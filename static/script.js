document.addEventListener("DOMContentLoaded", () => {
  const chessboard = document.getElementById("chessboard");
  const promotions = document.getElementById("promotions");
  const zoomInButton = document.getElementById("zoomIn");
  const zoomOutButton = document.getElementById("zoomOut");
  const restartButton = document.getElementById("restart");
  const prevButton = document.getElementById("prev");
  const nextButton = document.getElementById("next");
  const queenButton = document.getElementById("queen");
  const rookButton = document.getElementById("rook");
  const bishopButton = document.getElementById("bishop");
  const knightButton = document.getElementById("knight");
  const resultText = document.getElementById("result");
  const titleText = document.getElementById("title");
  const descText = document.getElementById("description");
  const myText = document.getElementById("myCardInfo");
  const opText = document.getElementById("opponentCardInfo");
  promotions.style.display = "none";
  resultText.innerText = "";
  titleText.innerText = "";
  descText.innerText = "";

  let size = chessboard.offsetWidth / 8;
  let messages = [];
  let cursor = -1;
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

  function prev() {
    if (cursor > 0) {
      cursor--;
      selected = null;
      targets = [];
      draw();
    }
  }

  function next() {
    if (cursor < messages.length - 1) {
      cursor++;
      selected = null;
      targets = [];
      draw();
    }
  }

  prevButton.addEventListener("click", prev);
  nextButton.addEventListener("click", next);

  document.addEventListener("keydown", event => {
    if (event.keyCode === 37) {
      prev();
    } else if (event.keyCode === 39) {
      next();
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
    return (messages[cursor].role.Player === "Black" ? (7 - file) : file) * size;
  }

  function rankToTop(rank) {
    return (messages[cursor].role.Player === "Black" ? rank : (7 - rank)) * size;
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

  const cardNames = [
    "효과 없음",
    "말에 오르다",
    "말에서 내리다",
    "융합",
    "분열",
    "합체",
    "산산조각",
    "교대 근무",
    "지구는 둥그니까",
    "전력 질주",
    "이사",
  ];

  function summarizeCards(cards, div) {
    const arr = Array(11).fill(0);
    for (card of cards)
      arr[card] += 1;
    for (let i = 0; i < 11; i++) {
      if (arr[i] > 0) {
        const p = document.createElement("p");
        p.innerText = `${cardNames[i]}: ${arr[i]}장`;
        div.appendChild(p);
      }
    }
  }

  function draw() {
    const message = messages[cursor];
    chessboard.style.width = `${8 * size}px`;
    chessboard.style.height = `${8 * size}px`;
    chessboard.innerHTML = "";
    resultText.innerText = "";
    titleText.innerText = "";
    descText.innerText = "";
    myText.innerHTML = "";
    opText.innerHTML = "";

    if (message) {
      for (let rank = 0; rank < 8; rank++) {
        for (let file = 0; file < 8; file++) {
          const piece = message.pieces[rank][file];
          if (piece)
            addPiece(pieceToFile(piece), file, rank);
        }
      }

      if (message.state === "Stalemate") {
        resultText.innerText = "스테일메이트";
      } else if (message.state !== "Normal") {
        const color = message.state.Checkmate === "White" ? "백" : "흑";
        resultText.innerText = `${color} 승리`;
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

      if (message.my_cards)
        summarizeCards(message.my_cards, myText);

      if (message.opponent_cards)
        summarizeCards(message.opponent_cards, opText);

      if (message.state === "Normal") {
        if (message.last_event) {
          if (message.last_event.Swap) {
            const ev = message.last_event.Swap;
            for (const e of ev) addOverlay(0, 0, 255, 0.2, e.file, e.rank);
            titleText.innerText = cardNames[7];
            descText.innerText = "두 기물의 위치가 바뀌었습니다.";
          } else if (message.last_event.KnightToBishop) {
            const ev = message.last_event.KnightToBishop;
            addOverlay(0, 0, 255, 0.2, ev.file, ev.rank);
            titleText.innerText = cardNames[2];
            descText.innerText = "나이트가 비숍이 되었습니다.";
          } else if (message.last_event.BishopToKnight) {
            const ev = message.last_event.BishopToKnight;
            addOverlay(0, 0, 255, 0.2, ev.file, ev.rank);
            titleText.innerText = cardNames[1];
            descText.innerText = "비숍이 나이트가 되었습니다.";
          } else if (message.last_event.RooksToQueen) {
            const ev = message.last_event.RooksToQueen;
            for (const e of ev) addOverlay(0, 0, 255, 0.2, e.file, e.rank);
            titleText.innerText = cardNames[3];
            descText.innerText = "룩 두 개가 퀸이 되었습니다.";
          } else if (message.last_event.QueenToRooks) {
            const ev = message.last_event.QueenToRooks;
            for (const e of ev) addOverlay(0, 0, 255, 0.2, e.file, e.rank);
            titleText.innerText = cardNames[4];
            descText.innerText = "퀸이 룩 두 개가 되었습니다.";
          } else if (message.last_event.PawnRun) {
            const ev = message.last_event.PawnRun;
            for (const e of ev) addOverlay(0, 0, 255, 0.2, e.file, e.rank);
            titleText.innerText = cardNames[9];
            descText.innerText = "폰이 앞으로 달려갔습니다.";
          } else if (message.last_event.PawnsToQueen) {
            const ev = message.last_event.PawnsToQueen;
            for (const e of ev) addOverlay(0, 0, 255, 0.2, e.file, e.rank);
            titleText.innerText = cardNames[5];
            descText.innerText = "폰 여덟 개가 퀸이 되었습니다.";
          } else if (message.last_event.QueenToPawns) {
            const ev = message.last_event.QueenToPawns;
            addOverlay(0, 0, 255, 0.2, ev[0].file, ev[0].rank);
            const rank = ev[1];
            for (let file = 0; file < 8; file++) {
              const pos = { file, rank };
              if (!samePos(ev[0], pos))
                addOverlay(0, 0, 255, 0.2, file, rank);
            }
            titleText.innerText = cardNames[6];
            descText.innerText = "퀸이 폰 여덟 개가 되었습니다.";
          } else if (message.last_event.Rotate) {
            const ev = message.last_event.Rotate;
            for (const e of ev) addOverlay(0, 0, 255, 0.2, e.file, e.rank);
            titleText.innerText = cardNames[8];
            descText.innerText = "기물이 체스판 반대편으로 건너갔습니다.";
          } else if (message.last_event.KingMove) {
            const ev = message.last_event.KingMove;
            for (const e of ev) addOverlay(0, 0, 255, 0.2, e.file, e.rank);
            titleText.innerText = cardNames[10];
            descText.innerText = "킹이 반대편으로 걸어갔습니다.";
          }
        } else if (message.last_card !== null) {
          if (message.last_card === 0) {
            titleText.innerText = cardNames[0];
          } else {
            titleText.innerText = "발동 실패";
            descText.innerText = cardNames[message.last_card];
          }
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
    if (messages[cursor].role.Player === "Black") {
      rank = 7 - rank;
      file = 7 - file;
    }
    return { rank, file };
  }

  function samePos(pos1, pos2) {
    return pos1.file === pos2.file && pos1.rank === pos2.rank;
  }

  chessboard.addEventListener("click", event => {
    if (messages[cursor] === null || cursor !== messages.length - 1 || waiting)
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
      const message = JSON.parse(ev.data);
      if (message.half_moves === 0) messages = [];
      messages.push(message);
      cursor = messages.length - 1;
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
    messages = [];
    cursor = -1;
    selected = null;
    targets = [];
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
