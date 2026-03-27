/**
 * WebSocket 클라이언트 JavaScript
 * Rust WebSocket 서버와 통신하는 웹 클라이언트
 */

// 전역 변수
let socket = null;
let isConnected = false;
let reconnectAttempts = 0;
const maxReconnectAttempts = 5;

/**
 * UI 상태를 업데이트하는 함수
 */
function updateUI() {
  const elements = {
    status: document.getElementById('status'),
    connectBtn: document.getElementById('connectBtn'),
    disconnectBtn: document.getElementById('disconnectBtn'),
    messageInput: document.getElementById('messageInput'),
    sendBtn: document.getElementById('sendBtn'),
    binaryBtn: document.getElementById('binaryBtn'),
    testBtn: document.getElementById('testBtn'),
  };

  if (isConnected) {
    elements.status.textContent = '연결됨';
    elements.status.className = 'status connected';
    elements.connectBtn.disabled = true;
    elements.disconnectBtn.disabled = false;
    elements.messageInput.disabled = false;
    elements.sendBtn.disabled = false;
    elements.binaryBtn.disabled = false;
    elements.testBtn.disabled = false;
  } else {
    elements.status.textContent = '연결 끊김';
    elements.status.className = 'status disconnected';
    elements.connectBtn.disabled = false;
    elements.disconnectBtn.disabled = true;
    elements.messageInput.disabled = true;
    elements.sendBtn.disabled = true;
    elements.binaryBtn.disabled = true;
    elements.testBtn.disabled = true;
  }
}

/**
 * 메시지를 화면에 추가하는 함수
 * @param {string} content - 메시지 내용
 * @param {string} type - 메시지 타입 ('sent', 'received', 'system')
 * @param {string} username - 사용자 이름
 * @param {string} timestamp - 타임스탬프
 */
function addMessage(content, type = 'received', username = '', timestamp = '') {
  const messages = document.getElementById('messages');
  const messageDiv = document.createElement('div');
  messageDiv.className = `message ${type}`;

  let html = '';

  // 타임스탬프 추가
  if (timestamp) {
    const date = new Date(timestamp);
    html += `<div class="timestamp">${date.toLocaleString('ko-KR')}</div>`;
  } else if (type !== 'system') {
    html += `<div class="timestamp">${new Date().toLocaleString('ko-KR')}</div>`;
  }

  // 사용자명 추가
  if (username) {
    html += `<span class="username">${escapeHtml(username)}:</span> `;
  }

  // 메시지 내용 추가
  html += escapeHtml(content);

  messageDiv.innerHTML = html;
  messages.appendChild(messageDiv);

  // 스크롤을 아래로 이동
  messages.scrollTop = messages.scrollHeight;

  // 메시지가 너무 많으면 오래된 것 제거 (성능 최적화)
  if (messages.children.length > 100) {
    messages.removeChild(messages.firstChild);
  }
}

/**
 * HTML 이스케이프 함수 (XSS 방지)
 * @param {string} text - 이스케이프할 텍스트
 * @returns {string} 이스케이프된 텍스트
 */
function escapeHtml(text) {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

/**
 * WebSocket 서버에 연결하는 함수
 */
function connect() {
  try {
    const serverUrl = 'ws://127.0.0.1:8080';
    addMessage(`${serverUrl}에 연결 시도 중...`, 'system');

    socket = new WebSocket(serverUrl);

    socket.onopen = function (event) {
      isConnected = true;
      reconnectAttempts = 0;
      updateUI();
      addMessage('서버에 연결되었습니다! 🎉', 'system');
      console.log('WebSocket 연결 성공:', event);
    };

    socket.onmessage = function (event) {
      handleIncomingMessage(event.data);
    };

    socket.onclose = function (event) {
      isConnected = false;
      updateUI();

      if (event.wasClean) {
        addMessage(
          `연결이 정상적으로 종료되었습니다. (코드: ${event.code}) 👋`,
          'system',
        );
      } else {
        addMessage(
          `연결이 비정상적으로 종료되었습니다. (코드: ${event.code}) ⚠️`,
          'system',
        );
        attemptReconnect();
      }

      console.log('WebSocket 연결 종료:', event);
    };

    socket.onerror = function (error) {
      addMessage('연결 오류가 발생했습니다: ' + error, 'system');
      console.error('WebSocket 오류:', error);
    };
  } catch (error) {
    addMessage('연결 실패: ' + error, 'system');
    console.error('연결 실패:', error);
  }
}

/**
 * 자동 재연결 시도
 */
function attemptReconnect() {
  if (reconnectAttempts < maxReconnectAttempts) {
    reconnectAttempts++;
    const delay = Math.min(1000 * Math.pow(2, reconnectAttempts), 10000);

    addMessage(
      `${delay / 1000}초 후 재연결 시도... (${reconnectAttempts}/${maxReconnectAttempts})`,
      'system',
    );

    setTimeout(() => {
      if (!isConnected) {
        connect();
      }
    }, delay);
  } else {
    addMessage('최대 재연결 시도 횟수에 도달했습니다.', 'system');
  }
}

/**
 * 들어오는 메시지를 처리하는 함수
 * @param {string|ArrayBuffer} data - 받은 데이터
 */
function handleIncomingMessage(data) {
  if (data instanceof ArrayBuffer) {
    // 바이너리 데이터 처리
    const uint8Array = new Uint8Array(data);
    const dataStr = Array.from(uint8Array).join(', ');
    addMessage(`바이너리 데이터 수신: [${dataStr}] 📁`, 'received');
  } else {
    // 텍스트 데이터 처리
    try {
      const messageData = JSON.parse(data);
      if (
        messageData.username &&
        messageData.content &&
        messageData.timestamp
      ) {
        // 구조화된 채팅 메시지
        addMessage(
          messageData.content,
          'received',
          messageData.username,
          messageData.timestamp,
        );
      } else {
        // 일반 JSON 데이터
        addMessage(JSON.stringify(messageData, null, 2), 'received');
      }
    } catch (e) {
      // 일반 텍스트 메시지
      addMessage(data, 'received');
    }
  }
}

/**
 * WebSocket 연결을 끊는 함수
 */
function disconnect() {
  if (socket) {
    socket.close(1000, '사용자 요청에 의한 연결 종료');
    socket = null;
  }
  reconnectAttempts = maxReconnectAttempts; // 자동 재연결 방지
}

/**
 * 메시지를 전송하는 함수
 */
function sendMessage() {
  const messageInput = document.getElementById('messageInput');
  const usernameInput = document.getElementById('usernameInput');

  const message = messageInput.value.trim();
  const username = usernameInput.value.trim() || '익명';

  if (!message) {
    addMessage('메시지를 입력해주세요.', 'system');
    return;
  }

  if (!socket || !isConnected) {
    addMessage('서버에 연결되지 않았습니다.', 'system');
    return;
  }

  try {
    const chatMessage = {
      username: username,
      content: message,
      timestamp: new Date().toISOString(),
    };

    socket.send(JSON.stringify(chatMessage));
    addMessage(message, 'sent', username, chatMessage.timestamp);
    messageInput.value = '';
    messageInput.focus();
  } catch (error) {
    addMessage('메시지 전송 실패: ' + error, 'system');
    console.error('메시지 전송 오류:', error);
  }
}

/**
 * 바이너리 데이터를 전송하는 함수
 */
function sendBinary() {
  if (!socket || !isConnected) {
    addMessage('서버에 연결되지 않았습니다.', 'system');
    return;
  }

  try {
    // "Hello World"를 바이트 배열로 변환
    const binaryData = new Uint8Array([
      72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100,
    ]);
    socket.send(binaryData.buffer);

    const dataStr = Array.from(binaryData).join(', ');
    addMessage(`바이너리 데이터 전송: [${dataStr}] 📁`, 'sent');
  } catch (error) {
    addMessage('바이너리 데이터 전송 실패: ' + error, 'system');
    console.error('바이너리 전송 오류:', error);
  }
}

/**
 * 테스트 메시지들을 순차적으로 전송하는 함수
 */
function sendTestMessages() {
  if (!socket || !isConnected) {
    addMessage('서버에 연결되지 않았습니다.', 'system');
    return;
  }

  const testMessages = [
    '안녕하세요! 웹 클라이언트에서 보내는 메시지입니다.',
    'WebSocket이 잘 작동하고 있네요! 🚀',
    'Rust 서버가 응답을 잘 보내주고 있습니다.',
    '자동 테스트 메시지 전송이 완료됩니다.',
  ];

  addMessage('테스트 메시지 전송을 시작합니다...', 'system');

  testMessages.forEach((msg, index) => {
    setTimeout(() => {
      if (socket && isConnected) {
        const messageInput = document.getElementById('messageInput');
        messageInput.value = msg;
        sendMessage();

        if (index === testMessages.length - 1) {
          setTimeout(() => {
            addMessage('테스트 메시지 전송이 완료되었습니다. ✅', 'system');
          }, 500);
        }
      }
    }, index * 1500);
  });
}

/**
 * 메시지를 모두 지우는 함수
 */
function clearMessages() {
  const messages = document.getElementById('messages');
  messages.innerHTML = '';
  addMessage('메시지가 모두 삭제되었습니다.', 'system');
}

/**
 * 키보드 이벤트 처리 함수
 * @param {KeyboardEvent} event - 키보드 이벤트
 */
function handleKeyPress(event) {
  if (event.key === 'Enter') {
    event.preventDefault();
    sendMessage();
  }
}

/**
 * 연결 상태를 확인하는 함수
 */
function checkConnectionStatus() {
  if (socket) {
    addMessage(
      `연결 상태: ${socket.readyState === WebSocket.OPEN ? '연결됨' : '연결 안됨'}`,
      'system',
    );
    addMessage(
      `Ready State: ${socket.readyState} (0:연결중, 1:열림, 2:닫는중, 3:닫힘)`,
      'system',
    );
  } else {
    addMessage('소켓이 생성되지 않았습니다.', 'system');
  }
}

// 페이지 로드 시 초기화
document.addEventListener('DOMContentLoaded', function () {
  updateUI();
  addMessage(
    'WebSocket 클라이언트가 준비되었습니다. 연결 버튼을 클릭하세요.',
    'system',
  );

  // 메시지 입력창에 포커스
  document
    .getElementById('messageInput')
    .addEventListener('keypress', handleKeyPress);

  // 개발자를 위한 디버그 함수 추가
  window.wsDebug = {
    checkStatus: checkConnectionStatus,
    getSocket: () => socket,
    isConnected: () => isConnected,
  };

  console.log(
    'WebSocket 클라이언트 로드 완료. wsDebug 객체를 통해 디버깅할 수 있습니다.',
  );
});
