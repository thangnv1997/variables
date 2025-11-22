const API_URL = '/api';

document.addEventListener('DOMContentLoaded', () => {
    loadMedicines();
    setupEventListeners();
});

function setupEventListeners() {
    document.getElementById('addForm').addEventListener('submit', async (e) => {
        e.preventDefault();
        const name = document.getElementById('name').value;
        const price = parseFloat(document.getElementById('price').value);
        const quantity = parseInt(document.getElementById('quantity').value);

        await addMedicine({ name, price, quantity });
        e.target.reset();
        loadMedicines();
    });

    document.getElementById('sellForm').addEventListener('submit', async (e) => {
        e.preventDefault();
        const id = parseInt(document.getElementById('sellId').value);
        const amount = parseInt(document.getElementById('sellAmount').value);

        await sellMedicine(id, amount);
        closeModal();
        loadMedicines();
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

function openSellModal(id) {
    document.getElementById('sellId').value = id;
    document.getElementById('sellModal').style.display = 'block';
}

function closeModal() {
    document.getElementById('sellModal').style.display = 'none';
    document.getElementById('sellForm').reset();
}
