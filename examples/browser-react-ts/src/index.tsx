import React from 'react'
import ReactDOM from 'react-dom'
import App from './App'

window.document.addEventListener('DOMContentLoaded', () => {
    const container = document.querySelector('#app')
    ReactDOM.render(<App />, container);
})
