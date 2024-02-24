<template>
  <div class="background">
    <h1>Crabs</h1>
    <div v-for="crab in crabs" :key="crab.id">
      <div v-if="!crab.editable">
        <div class="crab-card">
          <div class="crab-card-header">
            <button class="edit-icon" @click="toggleEdit(crab.id)">
              <i class="fas fa-pencil-alt fa-2xs"></i>
            </button>
            <button class="delete-icon" @click="deleteCrab(crab.id.id.String)">
              <i class="fas fa-times-circle fa-2xs"></i>
            </button>
          </div>
          <div class="crab-card-content">
            <h2>{{ crab.name }}</h2>
            <p>{{ crab.description }}</p>
          </div>
        </div>
      </div>
      <div v-else>
        <div class="crab-card">
          <div class="crab-card-header">
            <button class="save-icon" @click="updateCrab(crab)">
              <i class="fas fa-save"></i>
            </button>
          </div>
          <div class="crab-card-content">
            <input v-model="crab.name" placeholder="Enter name">
            <textarea v-model="crab.description" placeholder="Enter description"></textarea>
          </div>
        </div>
      </div>
    </div>
    <div class="crab-pictures">
      <div v-for="(_, index) in crabs" :key="index" class="crab-picture">
        <div class="crab-image-container">
          <img src="@/assets/crab.png" alt="Crab Image" class="crab-image">
        </div>
      </div>
    </div>
    <div class="add-crab-section">
      <input v-model="newCrab.name" placeholder="Enter name">
      <input v-model="newCrab.description" placeholder="Enter description">
      <button @click="addCrab">
        <i class="fas fa-plus"></i> Add Crab
      </button>
    </div>
  </div>
</template>

<script>
import { ref } from 'vue';
import axios from 'axios';

export default {
  name: 'App',
  setup() {
    const BASE_URL = process.env.VUE_APP_API_BASE_URL || 'http://localhost';
    const BASE_PORT = process.env.VUE_APP_API_BASE_PORT || '3000'
    const crabs = ref([]);
    const newCrab = ref({ name: '', description: '' });

    const fetchCrabs = () => {
      axios.get(`${BASE_URL}:${BASE_PORT}/api/v1/crabs`)
        .then(response => {
          crabs.value = response.data;
        })
        .catch(error => {
          console.error('Error fetching crabs:', error);
        });
    };

    const addCrab = () => {
      axios.post(`${BASE_URL}:${BASE_PORT}/api/v1/crabs`, newCrab.value)
        .then(() => {
          fetchCrabs(); // Refresh the list of crabs
          newCrab.value = { name: '', description: '' }; // Clear form fields
          positionCrabPictures();
        })
        .catch(error => {
          console.error('Error adding crab:', error);
        });
    };

    const deleteCrab = (id) => {
      axios.delete(`${BASE_URL}:${BASE_PORT}/api/v1/crabs/${id}`)
        .then(() => {
          fetchCrabs(); // Refresh the list of crabs after deletion
        })
        .catch(error => {
          console.error('Error deleting crab:', error);
        });
    };

    const updateCrab = (crab) => {
      axios.post(`${BASE_URL}:${BASE_PORT}/api/v1/crabs/${crab.id.id.String}`, crab)
        .then(() => {
          fetchCrabs(); // Refresh the list of crabs after update
        })
        .catch(error => {
          console.error('Error updating crab:', error);
        });
    };

    const toggleEdit = (crabId) => {
      const crab = crabs.value.find(crab => crab.id === crabId);
      if (crab) {
        crab.editable = !crab.editable;
      }
    };

    // Function to position crab pictures randomly
    const positionCrabPictures = () => {
      const crabPictures = document.querySelectorAll('.crab-picture');
      crabPictures.forEach(crabPicture => {
        const randomPositionTop = Math.random() * 80; // Random top position (0-80%)
        const randomPositionLeft = Math.random() * 80; // Random left position (0-80%)
        crabPicture.style.top = `${randomPositionTop}%`;
        crabPicture.style.left = `${randomPositionLeft}%`;
      });
    };

    fetchCrabs(); // Fetch initial list of crabs when component is mounted

    return {
      crabs,
      newCrab,
      addCrab,
      deleteCrab,
      updateCrab,
      toggleEdit,
      positionCrabPictures
    };
  }
};
</script>

<style scoped>
#app {
  font-family: Helvetica, sans-serif;
  width: 100vw;
  height: 100vh;
}

.background {
  width: 100vw;
  height: 100vh;
  background-image: url('/public/wave.svg');
  background-size: cover;
  background-position: center;
  overflow-y: auto;
}

h1 {
  text-align: center;
  color: white;
}

.crab-cards-container {
  margin: 0 auto;
  max-width: 1200px;
}

.crab-card {
  background-color: #fff;
  border: 2px solid #007bff;
  border-radius: 10px;
  margin: 0 auto 20px;
  padding: 15px;
  width: calc(33.33% - 20px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  transition: transform 0.3s ease-in-out;
}

.crab-card:hover {
  transform: scale(1.05);
}

.crab-card-content {
  display: flex;
  flex-direction: column;
}

.crab-card h2 {
  margin-bottom: 10px;
}

.add-crab-section {
  margin-top: 20px;
  text-align: center;
}

.add-crab-section input[type="text"],
.add-crab-section button {
  padding: 5px;
  border-radius: 5px;
  border: 1px solid #ccc;
}

.add-crab-section input[type="text"] {
  width: 200px;
  margin-right: 10px;
}

.add-crab-section button {
  background-color: #007bff;
  color: white;
  border: none;
  cursor: pointer;
}

.add-crab-section button:hover {
  background-color: #0056b3;
}

.crab-card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.delete-icon {
  position: absolute;
  top: 1px;
  right: 5px;
  cursor: pointer;
  color: red;
}

.delete-icon:hover {
  color: darkred;
  opacity: 1;
}

.edit-icon {
  position: absolute;
  top: 1px;
  left: 5px;
  cursor: pointer;
  color: blue;
}

.edit-icon:hover {
  color: darkblue;
  opacity: 1;
}

.edit-icon,
.delete-icon {
  background: none;
  border: none;
  cursor: pointer;
  padding: 0;
  font-size: 1.5em;
  opacity: 0.0;
  transition: opacity 0.3s;
}

.crab-pictures {
  position: absolute;
  top: 0;
  /* Choose either left or right positioning */
  left: 0;
  /* To position on the left */
  /* or */
  right: 0;
  /* To position on the right */
  /* Adjust as needed */
}

.crab-picture {
  position: absolute;
  bottom: 0;
  /* Align the bottom of crab pictures with the bottom of the crab cards */
}

.crab-image-container {
  position: relative;
  width: 50px;
  /* Adjust size as needed */
  height: auto;
  /* Maintain aspect ratio */
}

.crab-image {
  width: 100%;
  /* Ensure the image spans the whole container */
  height: auto;
  /* Maintain aspect ratio */
}
</style>
