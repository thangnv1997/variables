const API_URL = '/api';

document.addEventListener('DOMContentLoaded', () => {
    loadMedicines();
    loadImportBatches();
    loadExportBatches();
    setupEventListeners();
    setupTabs();
});

function setupTabs() {
    const tabBtns = document.querySelectorAll('.tab-btn');
    tabBtns.forEach(btn => {
        btn.addEventListener('click', () => {
            const targetTab = btn.dataset.tab;

            // Remove active class from all tabs and contents
            document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));

            // Add active class to clicked tab and corresponding content
            btn.classList.add('active');
            document.getElementById(targetTab).classList.add('active');
        });
    });
}

function setupEventListeners() {
    document.getElementById('addForm').addEventListener('submit', async (e) => {
        e.preventDefault();
        const name = document.getElementById('name').value;
        const price = parseFloat(document.getElementById('price').value);
        const quantity = parseInt(document.getElementById('quantity').value);

        await addMedicine({ name, price, quantity });
        e.target.reset();
        loadMedicines();
        loadImportBatches(); // Reload import history after adding
    });

    document.getElementById('sellForm').addEventListener('submit', async (e) => {
        e.preventDefault();
        const id = parseInt(document.getElementById('sellId').value);
        const amount = parseInt(document.getElementById('sellAmount').value);

        await sellMedicine(id, amount);
        closeModal();
        loadMedicines();
        loadExportBatches(); // Reload export history after selling
    });

    document.querySelector('.close').addEventListener('click', closeModal);
    window.addEventListener('click', (e) => {
        if (e.target == document.getElementById('sellModal')) {
            closeModal();
        }
    });
}

async function loadMedicines() {
    const res = await fetch(`${API_URL}/medicines`);
    const medicines = await res.json();
    const tbody = document.getElementById('medicineList');
    tbody.innerHTML = '';

    medicines.forEach(med => {
        const tr = document.createElement('tr');
        tr.innerHTML = `
            <td>${med.id}</td>
            <td>${med.name}</td>
            <td>${formatPrice(med.price)}</td>
            <td>${med.quantity}</td>
            <td class="actions">
                <button class="btn-sm btn-sell" onclick="openSellModal(${med.id})">Sell</button>
                <button class="btn-sm btn-delete" onclick="deleteMedicine(${med.id})">Delete</button>
            </td>
        `;
        tbody.appendChild(tr);
    });
}

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

async function addMedicine(medicine) {
    await fetch(`${API_URL}/medicines`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(medicine)
    });
}

async function deleteMedicine(id) {
    if (confirm('Are you sure you want to delete this medicine?')) {
        await fetch(`${API_URL}/medicines/${id}`, { method: 'DELETE' });
        loadMedicines();
    }
}

async function sellMedicine(id, amount) {
    const res = await fetch(`${API_URL}/sell`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id, amount })
    });

    if (!res.ok) {
        const err = await res.text();
        alert(err);
    }
}

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

function openSellModal(id) {
    document.getElementById('sellId').value = id;
    document.getElementById('sellModal').style.display = 'block';
}

function closeModal() {
    document.getElementById('sellModal').style.display = 'none';
    document.getElementById('sellForm').reset();
}
