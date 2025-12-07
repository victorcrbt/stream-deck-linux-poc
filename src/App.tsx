import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

function App() {
  const [count, setCount] = useState(0)


  const generateTiles = (size: number) => {
    return Array.from({ length: size }, (_, index) => index)
  }

  return (
    <div className="wrapper">
      <h1 className="title">Stream Deck</h1>

      <header className="header">
        <button>x</button>
      </header>

      <main className="tiles-container">
        {generateTiles(15).map((tile) => (
          <button key={tile} className="tile-button">
            {tile}
          </button>
        ))}
      </main>
    </div>
  );
}

export default App;
