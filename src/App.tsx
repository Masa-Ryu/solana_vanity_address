import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  interface Wallet {
    address: string;
    private_key: number[];
    private_key_str: string;
  }

  const [resultWallet, setResultWallet] = useState<Wallet | null>(null);
  const [prefix, setPrefix] = useState("");
  const [suffix, setSuffix] = useState("");
  const [numThreads, setThreads] = useState(1);
  const [isProcessing, setIsProcessing] = useState(false);

  async function handleProcess() {
    if (isProcessing) {
      return;
    }
    setIsProcessing(true);
    try {
      setResultWallet(await invoke("find_address", {
        prefix,
        suffix,
        numThreads
      }));
    } finally {
      setIsProcessing(false);
    }
  }

  function handleCopy(text: string) {
    navigator.clipboard.writeText(text);
  }

  return (
    <div className="container">
      <h1 className="title">Find Vanity Address on Solana!</h1>
      <div className="form-group">
        <label htmlFor="prefix">Prefix</label>
        <input
          id="prefix"
          value={prefix}
          onChange={(e) => setPrefix(e.currentTarget.value)}
          placeholder="アドレスの先頭に付けたいもの"
        />
      </div>
      <div className="form-group">
        <label htmlFor="suffix">Suffix</label>
        <input
          id="suffix"
          value={suffix}
          onChange={(e) => setSuffix(e.currentTarget.value)}
          placeholder="アドレスの末尾に付けたいもの"
        />
      </div>
      <div className="form-group">
        <label htmlFor="threads">使用するスレッド数</label>
        <input
          id="threads"
          type="number"
          value={numThreads}
          onChange={(e) => setThreads(Number(e.currentTarget.value))}
          placeholder="使用するスレッド数"
          min={1}
        />
      </div>
      <button type="button" className="primary-btn" onClick={handleProcess} disabled={isProcessing}>
        {isProcessing ? "処理中..." : "Go Searching"}
      </button>
      {resultWallet && (
        <div className="result">
          <div className="result-item">
            <p><strong>Address:</strong>
              <br />
              {resultWallet.address}</p>
            <button className="copy-btn" onClick={() => handleCopy(resultWallet.address)}>Copy</button>
          </div>
          <div className="result-item">
            <p><strong>Private Key (Bytes):</strong>
              <br />
              {resultWallet.private_key.join(", ")}</p>
            <button className="copy-btn" onClick={() => handleCopy(resultWallet.private_key.join(", "))}>Copy</button>
          </div>
          <div className="result-item">
            <p><strong>Private Key (String):</strong>
              <br />
              {resultWallet.private_key_str}</p>
            <button className="copy-btn" onClick={() => handleCopy(resultWallet.private_key_str)}>Copy</button>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
