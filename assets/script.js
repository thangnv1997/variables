const API_URL = '/api';

let warehouses = [];
let stockBatches = [];
let allStockBatches = [];

document.addEventListener('DOMContentLoaded', () => {
    loadWarehouses();
    loadStockBatches();
    loadTransfers();
    loadExpiringBatches();
    loadImportBatches();
    loadExportBatches();
    setupEventListeners();
    setupTabs();
});

function setupTabs() {
    // Main tabs
    const mainTabBtns = document.querySelectorAll('.main-tabs .tab-btn');
    mainTabBtns.forEach(btn => {
        btn.addEventListener('click', () => {
            const targetTab = btn.dataset.tab;

            document.querySelectorAll('.main-tabs .tab-btn').forEach(b => b.classList.remove('active'));
            document.querySelectorAll('.card > .tab-content').forEach(c => c.classList.remove('active'));

            btn.classList.add('active');
            document.getElementById(targetTab).classList.add('active');
        });
    });

    // Sub tabs (for history)
    const subTabBtns = document.querySelectorAll('.sub-tabs .tab-btn');
    subTabBtns.forEach(btn => {
        btn.addEventListener('click', () => {
            const targetTab = btn.dataset.tab;

            document.querySelectorAll('.sub-tabs .tab-btn').forEach(b => b.classList.remove('active'));
            document.querySelectorAll('#history > .tab-content').forEach(c => c.classList.remove('active'));

            btn.classList.add('active');
            document.getElementById(targetTab).classList.add('active');
        });
    });
}

function setupEventListeners() {
    document.getElementById('createWarehouseForm').addEventListener('submit', async (e) => {
        e.preventDefault();
        await createWarehouse();
    });

    document.getElementById('importBatchForm').addEventListener('submit', async (e) => {
        e.preventDefault();
        await importBatch();
    });

    document.getElementById('transferForm').addEventListener('submit', async (e) => {
        e.preventDefault();
        await transferBatch();
    });
}

// Warehouse Management
async function loadWarehouses() {
    const res = await fetch(`${API_URL}/warehouses`);
    warehouses = await res.json();
    renderWarehouses();
    populateWarehouseSelects();
}

function renderWarehouses() {
    const tbody = document.getElementById('warehouseList');
    tbody.innerHTML = '';

    warehouses.forEach(wh => {
        const batchCount = stockBatches.filter(b => b.warehouse_id === wh.id).length;
        const typeBadge = wh.warehouse_type === 'Main'
            ? '<span class="badge badge-main">Main</span>'
            : '<span class="badge badge-store">Store</span>';

        const tr = document.createElement('tr');
        tr.innerHTML = `
            <td>${wh.id}</td>
            <td>${wh.name}</td>
            <td>${typeBadge}</td>
            <td>${batchCount}</td>
        `;
        tbody.appendChild(tr);
    });
}

function populateWarehouseSelects() {
    const selects = [
        document.getElementById('importWarehouse'),
        document.getElementById('transferToWarehouse'),
        document.getElementById('warehouseFilter')
    ];

    selects.forEach(select => {
        if (!select) return;

        const currentValue = select.value;
        select.innerHTML = select.id === 'warehouseFilter' ? '<option value="">All Warehouses</option>' : '';

        warehouses.forEach(wh => {
            const option = document.createElement('option');
            option.value = wh.id;
            option.textContent = `${wh.name} (${wh.warehouse_type})`;
            select.appendChild(option);
        });

        if (currentValue) select.value = currentValue;
    });
}

async function createWarehouse() {
    const name = document.getElementById('warehouseName').value;
    const warehouse_type = document.getElementById('warehouseType').value;

    await fetch(`${API_URL}/warehouses`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, warehouse_type })
    });

    closeModal('createWarehouseModal');
    document.getElementById('createWarehouseForm').reset();
    loadWarehouses();
}

// Stock Batch Management
async function loadStockBatches() {
    const res = await fetch(`${API_URL}/stock-batches`);
    allStockBatches = await res.json();
    stockBatches = allStockBatches;
    renderStockBatches();
}

function renderStockBatches() {
    const tbody = document.getElementById('stockBatchList');
    tbody.innerHTML = '';

    stockBatches.forEach(batch => {
        const warehouse = warehouses.find(w => w.id === batch.warehouse_id);
        const expiryDate = new Date(batch.expiry_date);
        const daysLeft = Math.floor((expiryDate - new Date()) / (1000 * 60 * 60 * 24));

        let expiryClass = 'expiry-ok';
        if (daysLeft < 30) expiryClass = 'expiry-critical';
        else if (daysLeft < 90) expiryClass = 'expiry-warning';

        const tr = document.createElement('tr');
        tr.innerHTML = `
            <td>${batch.id}</td>
            <td>${batch.medicine_name}</td>
            <td>${warehouse ? warehouse.name : 'Unknown'}</td>
            <td>${batch.quantity}</td>
            <td>${formatPrice(batch.price)}</td>
            <td class="${expiryClass}">${formatDate(batch.expiry_date)}</td>
            <td>
                <button class="btn-sm btn-sell" onclick="openTransferModalForBatch(${batch.id})">Transfer</button>
            </td>
        `;
        tbody.appendChild(tr);
    });
}

function filterStockByWarehouse() {
    const warehouseId = document.getElementById('warehouseFilter').value;
    if (warehouseId === '') {
        stockBatches = allStockBatches;
    } else {
        stockBatches = allStockBatches.filter(b => b.warehouse_id == warehouseId);
    }
    renderStockBatches();
}

async function importBatch() {
    const medicine_id = parseInt(document.getElementById('importMedicineId').value);
    const medicine_name = document.getElementById('importMedicineName').value;
    const warehouse_id = parseInt(document.getElementById('importWarehouse').value);
    const quantity = parseInt(document.getElementById('importQuantity').value);
    const price = parseFloat(document.getElementById('importPrice').value);
    const expiryDateInput = document.getElementById('importExpiryDate').value;

    // Convert to ISO 8601 format
    const expiry_date = new Date(expiryDateInput).toISOString();

    const res = await fetch(`${API_URL}/import-batch`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ medicine_id, medicine_name, warehouse_id, quantity, price, expiry_date })
    });

    if (res.ok) {
        closeModal('importBatchModal');
        document.getElementById('importBatchForm').reset();
        loadStockBatches();
        loadImportBatches();
    } else {
        const error = await res.text();
        alert(error);
    }
}

// Transfer Management
async function loadTransfers() {
    const res = await fetch(`${API_URL}/transfers`);
    const transfers = await res.json();
    renderTransfers(transfers);
}

function renderTransfers(transfers) {
    const tbody = document.getElementById('transferList');
    tbody.innerHTML = '';

    transfers.reverse().forEach(transfer => {
        const fromWh = warehouses.find(w => w.id === transfer.from_warehouse_id);
        const toWh = warehouses.find(w => w.id === transfer.to_warehouse_id);

        const tr = document.createElement('tr');
        tr.innerHTML = `
            <td>${transfer.id}</td>
            <td>${transfer.medicine_name}</td>
            <td>${fromWh ? fromWh.name : 'Unknown'}</td>
            <td>${toWh ? toWh.name : 'Unknown'}</td>
            <td>${transfer.quantity}</td>
            <td>${formatDateTime(transfer.timestamp)}</td>
        `;
        tbody.appendChild(tr);
    });
}

async function transferBatch() {
    const batch_id = parseInt(document.getElementById('transferBatchId').value);
    const to_warehouse_id = parseInt(document.getElementById('transferToWarehouse').value);
    const quantity = parseInt(document.getElementById('transferQuantity').value);

    const res = await fetch(`${API_URL}/transfer-batch`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ batch_id, to_warehouse_id, quantity })
    });

    if (res.ok) {
        closeModal('transferModal');
        document.getElementById('transferForm').reset();
        loadStockBatches();
        loadTransfers();
    } else {
        const error = await res.text();
        alert(error);
    }
}

function openTransferModalForBatch(batchId) {
    document.getElementById('transferBatchId').value = batchId;
    openModal('transferModal');
}

// Expiring Batches
async function loadExpiringBatches() {
    const res = await fetch(`${API_URL}/expiring-batches`);
    const batches = await res.json();
    renderExpiringBatches(batches);
}

function renderExpiringBatches(batches) {
    const tbody = document.getElementById('expiringList');
    tbody.innerHTML = '';

    batches.forEach(batch => {
        const warehouse = warehouses.find(w => w.id === batch.warehouse_id);
        const expiryDate = new Date(batch.expiry_date);
        const daysLeft = Math.floor((expiryDate - new Date()) / (1000 * 60 * 60 * 24));

        let expiryClass = daysLeft < 30 ? 'expiry-critical' : 'expiry-warning';

        const tr = document.createElement('tr');
        tr.innerHTML = `
            <td>${batch.id}</td>
            <td>${batch.medicine_name}</td>
            <td>${warehouse ? warehouse.name : 'Unknown'}</td>
            <td>${batch.quantity}</td>
            <td class="${expiryClass}">${formatDate(batch.expiry_date)}</td>
            <td class="${expiryClass}">${daysLeft} days</td>
        `;
        tbody.appendChild(tr);
    });
}

// History
async function loadImportBatches() {
    const res = await fetch(`${API_URL}/batches/import`);
    const batches = await res.json();
    const tbody = document.getElementById('importList');
    tbody.innerHTML = '';

    batches.reverse().forEach(batch => {
        const tr = document.createElement('tr');
        tr.innerHTML = `
            <td>${batch.id}</td>
            <td>${batch.medicine_name}</td>
            <td>${batch.quantity}</td>
            <td>${formatPrice(batch.price)}</td>
            <td>${formatDateTime(batch.timestamp)}</td>
        `;
        tbody.appendChild(tr);
    });
}

async function loadExportBatches() {
    const res = await fetch(`${API_URL}/batches/export`);
    const batches = await res.json();
    const tbody = document.getElementById('exportList');
    tbody.innerHTML = '';

    batches.reverse().forEach(batch => {
        const tr = document.createElement('tr');
        tr.innerHTML = `
            <td>${batch.id}</td>
            <td>${batch.medicine_name}</td>
            <td>${batch.amount}</td>
            <td>${formatPrice(batch.price)}</td>
            <td>${formatDateTime(batch.timestamp)}</td>
        `;
        tbody.appendChild(tr);
    });
}

// Modal Management
function openModal(modalId) {
    document.getElementById(modalId).style.display = 'block';
}

function closeModal(modalId) {
    document.getElementById(modalId).style.display = 'none';
}

function openCreateWarehouseModal() {
    openModal('createWarehouseModal');
}

function openImportBatchModal() {
    openModal('importBatchModal');
}

function openTransferModal() {
    openModal('transferModal');
}

// Utility Functions
function formatPrice(price) {
    return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(price);
}

function formatDateTime(timestamp) {
    const date = new Date(timestamp);
    return date.toLocaleString('vi-VN', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit'
    });
}

function formatDate(timestamp) {
    const date = new Date(timestamp);
    return date.toLocaleDateString('vi-VN', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit'
    });
}

// Close modals when clicking outside
window.addEventListener('click', (e) => {
    if (e.target.classList.contains('modal')) {
        e.target.style.display = 'none';
    }
});
