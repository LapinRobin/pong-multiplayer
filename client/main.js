const ws = new WebSocket('ws://localhost:8080');

ws.addEventListener('open', function(event) {
    console.log('Connected to the WebSocket server');
});

ws.addEventListener('message', function(event) {
    console.log("Received:", event.data);
    const gameState = JSON.parse(event.data);
    

    leftPaddleY = gameState.leftPaddleY;
    rightPaddleY = gameState.rightPaddleY;

});


const canvas = document.getElementById('gameCanvas');
const ctx = canvas.getContext('2d');

let paddleHeight = 100, paddleWidth = 10;
let leftPaddleY = 300, rightPaddleY = 300;
let ballX = 400, ballY = 300;
let ballSpeedX = 5, ballSpeedY = 3;

document.addEventListener('keydown', function(event) {
    if (event.keyCode === 38) {  // Up arrow key
        ws.send(JSON.stringify({ action: 'move', direction: 'up' }));
    } else if (event.keyCode === 40) {  // Down arrow key
        ws.send(JSON.stringify({ action: 'move', direction: 'down' }));
    }
});




function gameLoop() {

    ballX += ballSpeedX;
    ballY += ballSpeedY;

    // Ball wall collision
    if(ballY <= 0 || ballY >= canvas.height) {
        ballSpeedY = -ballSpeedY;
    }

    if(ballX <= 0 || ballX >= canvas.width) {
        ballSpeedX = -ballSpeedX;
    }


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
