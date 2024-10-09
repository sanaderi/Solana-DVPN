const express = require('express')
// const { exec } = require('child_process');
// const anchor = require('@project-serum/anchor');
// const { PublicKey } = require('@solana/web3.js');
// const cron = require('node-cron');

// Solana setup
// const connection = new anchor.web3.Connection('https://api.mainnet-beta.solana.com', 'confirmed');
// const programId = new PublicKey('YourProgramIDHere');
// const idl = require('./idl.json'); // Your Anchor IDL file
// const provider = anchor.Provider.local();
// const program = new anchor.Program(idl, programId, provider);

// Initialize Express app
const app = express()
const port = 9090

app.use(express.json())

// Function to check username on Solana and create SSH user
// async function checkAndCreateUser(username) {
//     try {
//         // Check username on Solana blockchain
//         const userAccount = await program.account.user.fetch(username);
//         if (userAccount.expirationDate < Date.now() / 1000) {
//             return { success: false, message: 'User account expired' };
//         }

//         // Create SSH user
//         const password = Math.random().toString(36).slice(-8); // Generate a simple random password
//         exec(`sudo useradd -m ${username} && echo "${username}:${password}" | sudo chpasswd`, (error, stdout, stderr) => {
//             if (error) {
//                 console.error(`Error creating user: ${stderr}`);
//                 return;
//             }
//             console.log(`User ${username} created with password: ${password}`);
//         });

//         return { success: true, message: `User ${username} created successfully` };
//     } catch (error) {
//         return { success: false, message: 'Username not found or error occurred' };
//     }
// }

// Endpoint to handle SSH user creation requests
// app.post('/create-user', async (req, res) => {
//     const { username } = req.body;

//     if (!username) {
//         return res.status(400).json({ success: false, message: 'Username is required' });
//     }

//     const result = await checkAndCreateUser(username);
//     res.json(result);
// });

// Function to send "alive" message to Solana program
// async function sendAliveMessage() {
//     try {
//         // Define your logic to send an "alive" message or update status
//         // This is a placeholder. Replace with actual implementation.
//         console.log('Sending alive message to Solana program...');
//         const tx = await program.rpc.updateStatus({ /* your arguments here */ });
//         console.log('Alive message sent:', tx);
//     } catch (error) {
//         console.error('Error sending alive message:', error);
//     }
// }

// Schedule to send "alive" message every 10 minutes
// cron.schedule('*/10 * * * *', sendAliveMessage);

// Start the web server
app.listen(port, () => {
  console.log(`Server running on http://localhost:${port}`)
})

// Root route - display a simple message
app.get('/', (req, res) => {
  const responseObj = {
    message: 'Server is running on port 9090',
    status: 'success'
  }
  res.json(responseObj)
})
