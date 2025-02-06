import { initializeAgent, initializeCanister } from '@/cannister/Cannister';
import { useEffect, useState } from 'react';
import '@/styles/globals.css'
function App({ Component, pageProps }) {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const init = async () => {
      try {
        console.log("Initializing canister...");
        const canister = await initializeCanister();
        window.canister = canister;
        console.log("Canister initialized:", canister);
      } catch (err) {
        console.error("Failed to initialize canister:", err);
        setError(err);
      } finally {
        setLoading(false);
      }
    };

    init();
  }, []);

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error initializing canister: {error.message}</div>;

  return <Component {...pageProps} />;
}

export default App;
