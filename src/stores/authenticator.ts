import { defineStore } from 'pinia'
import axios from 'axios'

export const authStore = defineStore('auth', {
  state: () => ({
    user: null,
    token: null,
    isAuthenticated: false,
  }),
  actions: {
    async login(email: string, password: string) {
      try {
        const response = await axios.post('/api/login', { email, password })
        this.token = response.data.token
        this.user = response.data.user
        this.isAuthenticated = true
      } catch (error) {
        console.error('Login failed:', error)
        throw error
      }
    },
    logout() {
      this.token = null
      this.user = null
      this.isAuthenticated = false
    },
  },
})
