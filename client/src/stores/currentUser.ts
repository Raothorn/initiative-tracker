import { defineStore } from 'pinia'
import { ref, type Ref } from 'vue'

export const useUser = defineStore('user', () => {
    const userId: Ref<number> = ref(0)
    return { userId }
  })
  
  export default useUser