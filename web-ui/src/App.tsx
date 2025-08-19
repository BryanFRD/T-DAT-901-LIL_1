import { useState } from 'react'

import { Button } from './components/ui/button'

export default function App() {
  const [count, setCount] = useState<number>(0)

  const increment = () => {
    setCount(prevCount => prevCount + 1)
  }

  return (
    <div className='min-h-svh w-full flex items-center justify-center flex-col'>
      <Button onClick={increment}>count is {count}</Button>
    </div>
  )
}
