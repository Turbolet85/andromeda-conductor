import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import '@fontsource/jetbrains-mono/400.css'
import '@fontsource/jetbrains-mono/500.css'
import '@fontsource/ibm-plex-sans/400.css'
import '@fontsource/ibm-plex-sans/500.css'
import '@fontsource/ibm-plex-sans/600.css'
import './styles/tokens.css'
import App from './App'
import Gallery from './Gallery'

// DEV-only: visit with the #gallery hash to inspect the component primitives. The branch (and Gallery)
// tree-shake out of the production bundle — import.meta.env.DEV folds to false.
const showGallery = import.meta.env.DEV && window.location.hash === '#gallery'

createRoot(document.getElementById('root')!).render(
  <StrictMode>{showGallery ? <Gallery /> : <App />}</StrictMode>,
)
