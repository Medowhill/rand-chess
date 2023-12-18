document.addEventListener("DOMContentLoaded", () => {
  const chessboard = document.getElementById("chessboard");
  const zoomInButton = document.getElementById("zoomIn");
  const zoomOutButton = document.getElementById("zoomOut");
  const restartButton = document.getElementById("restart");
  const resultText = document.getElementById("result");

  let size = 70;
  let message = null;
  let selected = null;
  let targets = [];
  let waiting = true;

  zoomInButton.addEventListener('click', () => {
    size += 10;
    draw();
  });

  zoomOutButton.addEventListener('click', () => {
    if (size > 10) {
      size -= 10;
      draw();
    }
  });

  restartButton.addEventListener('click', () => {
    if (socket) {
      socket.send(JSON.stringify("Restart"));
      waiting = true;
    }
  });

  function pieceToFile(piece) {
    let s = piece.color === "White" ? "w" : "b";
    switch (piece.typ) {
      case "Pawn": s += "P"; break;
      case "Knight": s += "N"; break;
      case "Bishop": s += "B"; break;
      case "Rook": s += "R"; break;
      case "Queen": s += "Q"; break;
      case "King": s += "K"; break;
    }
    s += ".svg";
    return s;
  }

  function addPiece(fileName, file, rank) {
    const piece = document.createElement("div");
    piece.classList.add("chess-piece");
    piece.style.backgroundImage = `url('${fileName}')`;
    piece.style.width = `${size}px`;
    piece.style.height = `${size}px`;
    piece.style.left = `${file * size}px`;
    piece.style.top = `${(7 - rank) * size}px`;
    chessboard.appendChild(piece);
  }

  function addOverlay(r, g, b, file, rank) {
    const overlay = document.createElement('div');
    overlay.style.position = 'absolute';
    overlay.style.width = `${size}px`;
    overlay.style.height = `${size}px`;
    overlay.style.left = `${file * size}px`;
    overlay.style.top = `${(7 - rank) * size}px`;
    overlay.style.backgroundColor = `rgba(${r}, ${g}, ${b}, 0.2)`;
    overlay.style.display = 'block';
    chessboard.appendChild(overlay);
  }

  function draw() {
    chessboard.style.width = `${8 * size}px`;
    chessboard.style.height = `${8 * size}px`;
    chessboard.innerHTML = "";
    resultText.innerText = "";

    if (message) {
      for (let rank = 0; rank < 8; rank++) {
        for (let file = 0; file < 8; file++) {
          const piece = message.pieces[rank][file];
          if (piece)
            addPiece(pieceToFile(piece), file, rank);
        }
      }

      if (message.state === "Stalemate") {
        resultText.innerText = "Stalemate";
      } else if (message.state !== "Normal") {
        resultText.innerText = `${message.state.Checkmate} won`;
      }
    }
    
    if (selected) {
      addOverlay(0, 0, 0, selected.file, selected.rank);
      if (message) {
        for (const [loc, moves] of message.moves) {
          if (loc.file === selected.file && loc.rank === selected.rank) {
            targets = moves;
            for (const mv of targets) {
              addOverlay(255, 255, 0, mv.to.file, mv.to.rank);
            }
          }
        }
      }
    }
  }

  function getChessboardPosition(event) {
    const bounds = chessboard.getBoundingClientRect();
    const x = event.clientX - bounds.left;
    const y = event.clientY - bounds.top;
    const rank = 7 - Math.floor(y / size);
    const file = Math.floor(x / size);
    return { rank, file };
  }

  function samePos(pos1, pos2) {
    return pos1.file === pos2.file && pos1.rank === pos2.rank;
  }

  chessboard.addEventListener('click', event => {
    if (waiting)
      return;
    const pos = getChessboardPosition(event);
    if (selected) {
      if (samePos(pos, selected)) {
        selected = null;
      } else {
        let move = false;
        for (const mv of targets) {
          if (samePos(pos, mv.to)) {
            selected = null;
            targets = [];
            move = true;
            if (socket) {
              socket.send(JSON.stringify({ "Move": mv }));
              waiting = true;
            }
          }
        }
        if (!move)
          selected = pos;
      }
    } else {
      selected = pos;
    }
    draw();
  });

  let socket = null;

  function connect() {
    disconnect();

    const { location } = window;

    const proto = location.protocol.startsWith('https') ? 'wss' : 'ws';
    const wsUri = `${proto}://${location.host}/ws`;

    socket = new WebSocket(wsUri);
    socket.onopen = () => {
      console.log("connected");
    };
    socket.onmessage = (ev) => {
      message = JSON.parse(ev.data);
      waiting = false;
      draw();
    };
    socket.onclose = () => {
      console.log("disconnected");
      onclose();
    }
  }

  function onclose() {
    socket = null
    message = null;
    waiting = true;
    draw();
  }

  function disconnect() {
    if (socket) {
      socket.close()
      onclose();
    }
  }

  connect();
});
