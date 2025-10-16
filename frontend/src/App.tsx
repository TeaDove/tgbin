import React, { useState } from 'react';
import './App.css';
import { config } from './config';

function App() {
  const [text, setText] = useState('');
  const [user, setUser] = useState('@TeaDove');
  const [withCode, setWithCode] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [toast, setToast] = useState<{ message: string, type: 'success' | 'error' } | null>(null);

  const showToast = (message: string, type: 'success' | 'error') => {
    setToast({ message, type });
    setTimeout(() => setToast(null), 5000);
  };

  const handlePasteText = async () => {
    if (isLoading) return;

    setIsLoading(true);
    try {
      const response = await fetch(`${config.url}/paste/text`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          user: user,
          text: text,
          withCode: withCode
        }),
      });

      const responseBody = await response.text();
      let parsedBody;
      try {
        parsedBody = JSON.parse(responseBody);
      } catch {
        parsedBody = responseBody;
      }

      if (response.ok) {
        showToast(`Success (${response.status}): ${JSON.stringify(parsedBody)}`, 'success');
        console.log('Success:', parsedBody);
      } else {
        showToast(`Error (${response.status}): ${JSON.stringify(parsedBody)}`, 'error');
        console.error('Error:', response.statusText);
      }
    } catch (error) {
      showToast(`Network Error: ${error}`, 'error');
      console.error('Error:', error);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="App">
      {toast && (
        <div className={`toast toast-${toast.type}`}>
          {toast.message}
        </div>
      )}

      <div className="main-container">
        <div className="left-panel">
          <div className="form-group">
            <label className="checkbox-label">
              <input
                type="checkbox"
                checked={withCode}
                onChange={(e) => setWithCode(e.target.checked)}
              />
              <span>add &lt;code&gt;</span>
            </label>
          </div>

          <div className="form-group">
            <label>user</label>
            <input
              type="text"
              value={user}
              onChange={(e) => setUser(e.target.value)}
              className="user-input"
            />
          </div>

          <button
            onClick={handlePasteText}
            className={`paste-button ${isLoading ? 'loading' : ''}`}
            disabled={isLoading}
          >
            {isLoading ? 'Sending...' : 'send'}
          </button>
        </div>

        <div className="right-panel">
          <textarea
            value={text}
            onChange={(e) => setText(e.target.value)}
            className="text-input"
            placeholder="Enter your text here..."
          />
        </div>
      </div>
    </div>
  );
}

export default App;
