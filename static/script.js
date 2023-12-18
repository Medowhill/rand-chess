document.addEventListener("DOMContentLoaded", () => {
  const chessboard = document.getElementById("chessboard");
  const promotions = document.getElementById("promotions");
  const zoomInButton = document.getElementById("zoomIn");
  const zoomOutButton = document.getElementById("zoomOut");
  const restartButton = document.getElementById("restart");
  const queenButton = document.getElementById("queen");
  const rookButton = document.getElementById("rook");
  const bishopButton = document.getElementById("bishop");
  const knightButton = document.getElementById("knight");
  const resultText = document.getElementById("result");
  const titleText = document.getElementById("title");
  const descText = document.getElementById("description");
  promotions.style.display = "none";
  resultText.innerText = "";
  titleText.innerText = "";
  descText.innerText = "";

  let size = 70;
  let message = null;
  let selected = null;
  let targets = [];
  let waiting = true;
  let promotionMove = null;

  zoomInButton.addEventListener("click", () => {
    size += 10;
    draw();
  });

  zoomOutButton.addEventListener("click", () => {
    if (size > 10) {
      size -= 10;
      draw();
    }
  });

  restartButton.addEventListener("click", () => {
    if (socket) {
      socket.send(JSON.stringify("Restart"));
      waiting = true;
    }
  });

  function promotionClick(event) {
    promotions.style.display = "none";
    if (promotionMove) {
      promotionMove.promote_to = event.target.innerText;
      selected = null;
      targets = [];
      move = true;
      if (socket) {
        socket.send(JSON.stringify({ "Move": promotionMove }));
        waiting = true;
      }
    }
  }

  queenButton.addEventListener("click", promotionClick);
  rookButton.addEventListener("click", promotionClick);
  bishopButton.addEventListener("click", promotionClick);
  knightButton.addEventListener("click", promotionClick);

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

  function fileToLeft(file) {
    return (message.role.Player === "Black" ? (7 - file) : file) * size;
  }

  function rankToTop(rank) {
    return (message.role.Player === "Black" ? rank : (7 - rank)) * size;
  }

  function addPiece(fileName, file, rank) {
    const piece = document.createElement("div");
    piece.classList.add("chess-piece");
    piece.style.backgroundImage = `url("${fileName}")`;
    piece.style.width = `${size}px`;
    piece.style.height = `${size}px`;
    piece.style.left = `${fileToLeft(file)}px`;
    piece.style.top = `${rankToTop(rank)}px`;
    chessboard.appendChild(piece);
  }

  function addOverlay(r, g, b, a, file, rank) {
    const overlay = document.createElement("div");
    overlay.style.position = "absolute";
    overlay.style.width = `${size}px`;
    overlay.style.height = `${size}px`;
    overlay.style.left = `${fileToLeft(file)}px`;
    overlay.style.top = `${rankToTop(rank)}px`;
    overlay.style.display = "block";
    overlay.style.backgroundColor = `rgba(${r}, ${g}, ${b}, ${a})`;
    chessboard.appendChild(overlay);
  }

  function addCircle(r, g, b, a, s, file, rank) {
    const overlay = document.createElement("div");
    overlay.style.position = "absolute";
    overlay.style.width = `${size * s}px`;
    overlay.style.height = `${size * s}px`;
    overlay.style.left = `${fileToLeft(file) + (1 - s) * size / 2}px`;
    overlay.style.top = `${rankToTop(rank) + (1 - s) * size / 2}px`;
    overlay.style.display = "block";
    overlay.style.borderRadius = "50%";
    overlay.style.backgroundColor = `rgba(${r}, ${g}, ${b}, ${a})`;
    chessboard.appendChild(overlay);
  }

  function draw() {
    chessboard.style.width = `${8 * size}px`;
    chessboard.style.height = `${8 * size}px`;
    chessboard.innerHTML = "";
    resultText.innerText = "";
    titleText.innerText = "";
    descText.innerText = "";

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

      if (message.last) {
        const from = message.last.from;
        addOverlay(255, 255, 0, 0.2, from.file, from.rank);
        const to = message.last.to;
        addOverlay(255, 255, 0, 0.2, to.file, to.rank);
      }

      if (message.check) {
        addOverlay(255, 0, 0, 0.2, message.check.file, message.check.rank);
      }

      if (message.last_event) {
        if (message.last_event.Swap) {
          const ev = message.last_event.Swap;
          addOverlay(0, 0, 255, 0.2, ev[0].file, ev[0].rank);
          addOverlay(0, 0, 255, 0.2, ev[1].file, ev[1].rank);
          titleText.innerText = "교대";
          descText.innerText = "킹이 아닌 두 기물의 위치가 바뀌었습니다.";
        } else if (message.last_event.KnightToBishop) {
          const ev = message.last_event.KnightToBishop;
          addOverlay(0, 0, 255, 0.2, ev.file, ev.rank);
          titleText.innerText = "말에서 내리기";
          descText.innerText = "나이트가 비숍이 되었습니다.";
        } else if (message.last_event.BishopToKnight) {
          const ev = message.last_event.BishopToKnight;
          addOverlay(0, 0, 255, 0.2, ev.file, ev.rank);
          titleText.innerText = "말에 타기";
          descText.innerText = "비숍이 나이트가 되었습니다.";
        } else if (message.last_event.RooksToQueen) {
          const ev = message.last_event.RooksToQueen;
          addOverlay(0, 0, 255, 0.2, ev[0].file, ev[0].rank);
          addOverlay(0, 0, 255, 0.2, ev[1].file, ev[1].rank);
          titleText.innerText = "융합";
          descText.innerText = "룩 두 개가 퀸이 되었습니다.";
        } else if (message.last_event.QueenToRooks) {
          const ev = message.last_event.QueenToRooks;
          addOverlay(0, 0, 255, 0.2, ev[0].file, ev[0].rank);
          addOverlay(0, 0, 255, 0.2, ev[1].file, ev[1].rank);
          titleText.innerText = "분열";
          descText.innerText = "퀸이 룩 두 개가 되었습니다.";
        } else if (message.last_event.PawnRun) {
          const ev = message.last_event.PawnRun;
          addOverlay(0, 0, 255, 0.2, ev[0].file, ev[0].rank);
          addOverlay(0, 0, 255, 0.2, ev[1].file, ev[1].rank);
          titleText.innerText = "전력질주";
          descText.innerText = "폰이 앞으로 달려나갔습니다.";
        }
      }
    }
    
    if (selected) {
      addOverlay(255, 255, 0, 0.2, selected.file, selected.rank);
      if (message) {
        for (const [loc, moves] of message.moves) {
          if (loc.file === selected.file && loc.rank === selected.rank) {
            targets = moves;
            for (const mv of targets) {
              const s = (mv.attack && samePos(mv.attack[0], mv.to)) ? 0.8 : 0.33;
              addCircle(180, 180, 180, 0.7, s, mv.to.file, mv.to.rank);
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
    let rank = 7 - Math.floor(y / size);
    let file = Math.floor(x / size);
    if (message.role.Player === "Black") {
      rank = 7 - rank;
      file = 7 - file;
    }
    return { rank, file };
  }

  function samePos(pos1, pos2) {
    return pos1.file === pos2.file && pos1.rank === pos2.rank;
  }

  chessboard.addEventListener("click", event => {
    if (message === null || waiting)
      return;
    promotions.style.display = "none";
    const pos = getChessboardPosition(event);
    if (selected) {
      if (samePos(pos, selected)) {
        selected = null;
      } else {
        let move = false;
        for (const mv of targets) {
          if (samePos(pos, mv.to)) {
            move = true;
            if (mv.is_promotion) {
              promotions.style.display = "block";
              promotionMove = mv;
            } else {
              selected = null;
              targets = [];
              if (socket) {
                socket.send(JSON.stringify({ "Move": mv }));
                waiting = true;
              }
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

    const proto = location.protocol.startsWith("https") ? "wss" : "ws";
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
    };
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
