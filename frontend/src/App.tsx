import React from 'react';
import './App.css';
import { config } from './config';

function App() {
  const handlePasteText = async () => {
    try {
      const response = await fetch(`${config.url}/api/paste/text`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          user: "teadove",
          text: "example",
          withCode: true
        }),
      });

      if (response.ok) {
        const data = await response.json();
        console.log('Success:', data);
      } else {
        console.error('Error:', response.statusText);
      }
    } catch (error) {
      console.error('Error:', error);
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>Hello World</h1>
        <button onClick={handlePasteText} className="paste-button">
          paste-text
        </button>
      </header>
    </div>
  );
}

export default App;
