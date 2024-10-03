// main.js
let currentData = [];
let filteredData = [];

async function applyFilters() {
    const filters = {
        search: elements.search.value.toLowerCase(),
        pharmacy: elements.pharmacy.value,
        region: elements.region.value,
        month: elements.month.value,
        method: elements.method.value,
        unit: elements.unit.value
    };

    try {
        // Call Rust backend for filtering
        const response = await fetch('/api/filter', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(filters)
        });

        if (!response.ok) {
            throw new Error('Filter request failed');
        }

        filteredData = await response.json();

        // Get stats from backend
        const statsResponse = await fetch('/api/stats', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(filteredData)
        });

        if (!statsResponse.ok) {
            throw new Error('Stats request failed');
        }

        const stats = await statsResponse.json();

        // Update UI with stats
        elements.totalUnits.textContent = stats.total_units.toLocaleString();
        elements.totalMeds.textContent = stats.unique_meds.toLocaleString();
        elements.totalPharmacies.textContent = stats.unique_pharmacies.toLocaleString();

        renderTable();
        updateFilters();
    } catch (error) {
        console.error('Error applying filters:', error);
    }
}

// Update Firebase integration to work with the backend
async function loadInitialData() {
    showLoading();
    try {
        const snapshot = await currentApp.database().ref('pharmacyData').once('value');
        const data = snapshot.val();
        if (!data) {
            throw new Error('No data available');
        }
        
        currentData = Object.values(data).flat();
        await applyFilters();
    } catch (error) {
        console.error('Error loading data:', error);
        elements.tableBody.innerHTML = `
            <tr>
                <td colspan="7" class="empty-state">
                    Error loading data: ${error.message}. Please try again.
                </td>
            </tr>
        `;
    } finally {
        hideLoading();
    }
}
