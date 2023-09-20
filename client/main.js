const ws = new WebSocket('ws://localhost:8080');

ws.addEventListener('open', function(event) {
    console.log('Connected to the WebSocket server');
});

ws.addEventListener('message', function(event) {
    console.log("Received:", event.data);
    const gameState = JSON.parse(event.data);

    leftPaddleY = gameState.leftPaddleY;
    rightPaddleY = gameState.rightPaddleY;
    ballX = gameState.ballX;
    ballY = gameState.ballY;
    // We are updating ballX and ballY based on the server's game state
});

const canvas = document.getElementById('gameCanvas');
const ctx = canvas.getContext('2d');

let paddleHeight = 100, paddleWidth = 10;
let leftPaddleY, rightPaddleY;  // Initial values will be set by the server
let ballX, ballY;  // Initial values will be set by the server


document.addEventListener('keydown', function(event) {
    if (event.keyCode === 38) {  // Up arrow key
        ws.send(JSON.stringify({ action: 'move', direction: 'up' }));
    } else if (event.keyCode === 40) {  // Down arrow key
        ws.send(JSON.stringify({ action: 'move', direction: 'down' }));
    }
});

function gameLoop() {
    // Removed the local ball position updates; they're now managed by the server

    draw();

    requestAnimationFrame(gameLoop);
}

function draw() {
    // Clear canvas
    ctx.fillStyle = '#000';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Draw paddles
    ctx.fillStyle = '#fff';
    ctx.fillRect(0, leftPaddleY, paddleWidth, paddleHeight);
    ctx.fillRect(canvas.width - paddleWidth, rightPaddleY, paddleWidth, paddleHeight);

    // Draw ball
    ctx.beginPath();
    ctx.arc(ballX, ballY, 10, 0, Math.PI*2);
    ctx.fillStyle = '#fff';
    ctx.fill();
    ctx.closePath();
}

gameLoop();
