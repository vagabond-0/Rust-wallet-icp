import { useState, useEffect } from 'react';
import { Principal } from '@dfinity/principal';
import { motion } from 'framer-motion';

export default function WalletApp() {
  const [username, setUsername] = useState('');
  const [balance, setBalance] = useState(0);
  const [transferAmount, setTransferAmount] = useState('');
  const [recipientId, setRecipientId] = useState('');
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  const [userInfo, setUserInfo] = useState(null);

  const handleCreateAccount = async () => {
    try {
      setError('');
      const user = { username, balance: BigInt(0) };
      const response = await window.canister.create_account(user);
      if (response.length > 0) {
        setUserInfo(response[0]);
        setSuccess('Account created successfully!');
      }
    } catch (err) {
      setError('Failed to create account: ' + err.message);
    }
  };

  const handleTransfer = async () => {
    try {
      setError('');
      const amount = BigInt(transferAmount);
      const recipient = Principal.fromText(recipientId);
      const result = await window.canister.transfer_tokens(recipient, amount);
      if ('Ok' in result) {
        setSuccess('Transfer successful!');
        fetchBalance();
      } else {
        setError(result.Err);
      }
    } catch (err) {
      setError('Transfer failed: ' + err.message);
    }
  };

  const fetchBalance = async () => {
    try {
      const balance = await window.canister.get_balance();
      setBalance(Number(balance));
    } catch (err) {
      setError('Failed to fetch balance');
    }
  };

  const fetchUserInfo = async () => {
    try {
      const user = await window.canister.get_self();
      setUserInfo(user);
    } catch (err) {
      console.error('Failed to fetch user info');
    }
  };

  useEffect(() => {
    fetchUserInfo();
    fetchBalance();
  }, []);

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-100 p-4">
      <motion.div 
        className="max-w-md w-full bg-white shadow-lg rounded-xl p-6"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.5 }}
      >
        <h1 className="text-3xl font-bold text-center text-blue-600 mb-6">ICP Wallet</h1>
        {error && <div className="bg-red-100 border-l-4 border-red-500 text-red-700 p-3 mb-4">{error}</div>}
        {success && <div className="bg-green-100 border-l-4 border-green-500 text-green-700 p-3 mb-4">{success}</div>}
        
        {!userInfo ? (
          <div className="space-y-4">
            <h2 className="text-xl font-semibold">Create Account</h2>
            <input
              type="text"
              placeholder="Username"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              className="w-full px-4 py-2 border rounded-md focus:ring-2 focus:ring-blue-400"
            />
            <button
              onClick={handleCreateAccount}
              className="w-full bg-blue-600 text-white py-2 rounded-md hover:bg-blue-700 transition"
            >
              Create Account
            </button>
          </div>
        ) : (
          <div className="space-y-6">
            <div className="bg-gray-100 p-4 rounded-lg shadow">
              <p className="text-gray-600">Welcome</p>
              <p className="text-xl font-semibold text-black">{userInfo.username} </p>
              <p className="text-gray-600 mt-2">Balance</p>
              <p className="text-xl font-semibold text-black">{balance} tokens</p>
            </div>

            <div className="space-y-4">
              <h2 className="text-xl font-semibold">Transfer Tokens</h2>
              <input
                type="text"
                placeholder="Recipient Principal ID"
                value={recipientId}
                onChange={(e) => setRecipientId(e.target.value)}
                className="w-full px-4 py-2 border rounded-md focus:ring-2 focus:ring-blue-400"
              />
              <input
                type="number"
                placeholder="Amount"
                value={transferAmount}
                onChange={(e) => setTransferAmount(e.target.value)}
                className="w-full px-4 py-2 border rounded-md focus:ring-2 focus:ring-blue-400"
              />
              <button
                onClick={handleTransfer}
                className="w-full bg-blue-600 text-white py-2 rounded-md hover:bg-blue-700 transition"
              >
                Transfer
              </button>
            </div>
          </div>
        )}
      </motion.div>
    </div>
  );
}
