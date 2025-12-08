import './App.css'
import { MdSettings } from 'react-icons/md'

import micOnActive from './assets/images/icons/mic-on.png';
import micOffActive from './assets/images/icons/mic-off.png';
import g560Active from './assets/images/icons/g560-active.png';
import hs80InputActive from './assets/images/icons/hs80-input-active.png';
import hs80OutputActive from './assets/images/icons/hs80-output-active.png';
import hyperxQuadcastActive from './assets/images/icons/hyperx-quadcast-active.png';
import wh1000xm3Active from './assets/images/icons/wh1000xm3-active.png';

const images = {
  0: micOnActive,
  5: g560Active,
  6: hs80OutputActive,
  7: wh1000xm3Active,
  10: hyperxQuadcastActive,
  11: hs80InputActive,
}

function App() {
  const generateTiles = (size: number) => {
    return Array.from({ length: size }, (_, index) => index)
  }

  return (
    <div className="wrapper">
      <div className="tittle-wrapper">
        <h1 className="title">Stream Deck</h1>
      </div>

      <header className="header">
        <button><MdSettings size={24} color="#ccc"/></button>
      </header>

      <main className="tiles-container">
        {generateTiles(15).map((tile) => (
          <button key={tile} className="tile-button" style={{ backgroundImage: `url(${images[tile as keyof typeof images]})` }}>
            {tile}
          </button>
        ))}
      </main>
    </div>
  );
}

export default App;
