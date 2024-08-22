import $socket from '@/socket'
import { defineStore } from 'pinia'
import { computed, ref, type Ref } from 'vue'

export const useUser = defineStore('user', () => {
    // Data
    const userId: Ref<number> = ref(0)
    const username: Ref<string | null> = ref(null)

    // Getters
    const isLoggedIn = computed(() => {
      return username.value !== null;
    })

    // Mutators
    function login(_username: string)  {
      // TODO wait for confirmation from server before setting username on client side
      $socket.sendLogin(_username)
      username.value = _username
    }

    return { userId, username, login, isLoggedIn }
  })
  
  export default useUser