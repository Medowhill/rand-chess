document.addEventListener('DOMContentLoaded', function() {
    const chessboard = document.getElementById('chessboard');
    const zoomInButton = document.getElementById('zoomIn');
    const zoomOutButton = document.getElementById('zoomOut');

    let size = 70;
    let prevSize = size;

    chessboard.style.width = `${8 * size}px`;
    chessboard.style.height = `${8 * size}px`;

    function addPiece(fileName, x, y) {
        const piece = document.createElement('div');
        piece.classList.add('chess-piece');
        piece.style.backgroundImage = `url('${fileName}')`;
        piece.style.width = `${size}px`;
        piece.style.height = `${size}px`;
        piece.style.left = `${x * size}px`;
        piece.style.top = `${y * size}px`;
        chessboard.appendChild(piece);
    }

    function setupChessboard() {
        // Clear existing pieces
        chessboard.innerHTML = '';

        // Place pieces in initial positions
        // Black pieces
        for (let i = 0; i < 8; i++) { addPiece('bP.svg', i, 1); } // Pawns
        addPiece('bR.svg', 0, 0); addPiece('bR.svg', 7, 0); // Rooks
        addPiece('bN.svg', 1, 0); addPiece('bN.svg', 6, 0); // Knights
        addPiece('bB.svg', 2, 0); addPiece('bB.svg', 5, 0); // Bishops
        addPiece('bQ.svg', 3, 0); // Queen
        addPiece('bK.svg', 4, 0); // King

        // White pieces
        for (let i = 0; i < 8; i++) { addPiece('wP.svg', i, 6); } // Pawns
        addPiece('wR.svg', 0, 7); addPiece('wR.svg', 7, 7); // Rooks
        addPiece('wN.svg', 1, 7); addPiece('wN.svg', 6, 7); // Knights
        addPiece('wB.svg', 2, 7); addPiece('wB.svg', 5, 7); // Bishops
        addPiece('wQ.svg', 3, 7); // Queen
        addPiece('wK.svg', 4, 7); // King
    }

    setupChessboard(); // Initial setup

    let overlay = document.createElement('div');
    overlay.style.position = 'absolute';
    overlay.style.width = `${size}px`;
    overlay.style.height = `${size}px`;
    overlay.style.backgroundColor = 'rgba(0, 0, 0, 0.2)'; // Semi-transparent black
    overlay.style.display = 'none';
    chessboard.appendChild(overlay);

    function getChessboardPosition(event) {
        const bounds = chessboard.getBoundingClientRect();
        const x = event.clientX - bounds.left;
        const y = event.clientY - bounds.top;
        const rank = Math.floor(y / size);
        const file = Math.floor(x / size);
        return { rank, file };
    }

    chessboard.addEventListener('click', function(event) {
        const position = getChessboardPosition(event);

        overlay.style.left = `${position.file * size}px`;
        overlay.style.top = `${position.rank * size}px`;
        overlay.style.display = 'block';
    });

    function adjustOverlay() {
        overlay.style.width = `${size}px`;
        overlay.style.height = `${size}px`;
        overlay.style.left = `${parseInt(overlay.style.left) / prevSize * size}px`;
        overlay.style.top = `${parseInt(overlay.style.top) / prevSize * size}px`;
    }

    function adjustPieces() {
        document.querySelectorAll('.chess-piece').forEach(piece => {
            piece.style.width = `${size}px`;
            piece.style.height = `${size}px`;
            piece.style.left = `${parseInt(piece.style.left) / prevSize * size}px`;
            piece.style.top = `${parseInt(piece.style.top) / prevSize * size}px`;
        });
    }

    zoomInButton.addEventListener('click', function() {
        size += 10;
        chessboard.style.width = `${size * 8}px`;
        chessboard.style.height = `${size * 8}px`;
        adjustPieces();
        adjustOverlay();
        prevSize = size;
    });

    zoomOutButton.addEventListener('click', function() {
        if (size > 10) {
            size -= 10;
            chessboard.style.width = `${size * 8}px`;
            chessboard.style.height = `${size * 8}px`;
            adjustPieces();
            adjustOverlay();
            prevSize = size;
        }
    });
});
