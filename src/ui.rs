pub fn index_html() -> &'static str {
    r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>StoryVote</title>
  <style>
    :root {
      color-scheme: light;
      --bg: #f6f8fb;
      --bg-2: #dbe8ff;
      --panel: #ffffff;
      --text: #1c2430;
      --muted: #5e6b7d;
      --accent: #0b5fff;
      --accent-2: #0847c2;
      --secondary: #6c757d;
      --input-bg: #ffffff;
      --code-bg: #eef3fb;
      --good: #198754;
      --warn: #c0392b;
      --border: #d8e0ea;
      --card-bg: #eff4ff;
      --card-border: #b8ccff;
      --card-text: #123a8a;
      --shadow: 0 8px 20px rgba(17, 37, 70, 0.06);
      --pill-host-bg: #e8f2ff;
      --pill-host-text: #0b5fff;
      --pill-voted-bg: #eaf7ef;
      --pill-value-bg: #fff3db;
      --pill-value-text: #8a5a00;
      --theme-select-bg: #ffffff;
    }
    :root[data-theme="dark"] {
      color-scheme: dark;
      --bg: #0e1420;
      --bg-2: #1a2538;
      --panel: #151f31;
      --text: #e6edf7;
      --muted: #a3b2c7;
      --accent: #4f8bff;
      --accent-2: #2d6be4;
      --secondary: #45566f;
      --input-bg: #0f1727;
      --code-bg: #1f2a3f;
      --good: #46ba78;
      --warn: #ff8578;
      --border: #2f3d56;
      --card-bg: #11203d;
      --card-border: #38548f;
      --card-text: #a8c6ff;
      --shadow: 0 8px 24px rgba(0, 0, 0, 0.35);
      --pill-host-bg: #20345f;
      --pill-host-text: #a5c0ff;
      --pill-voted-bg: #203e31;
      --pill-value-bg: #47381d;
      --pill-value-text: #ffd58a;
      --theme-select-bg: #0f1727;
    }
    @media (prefers-color-scheme: dark) {
      :root:not([data-theme="light"]) {
        color-scheme: dark;
        --bg: #0e1420;
        --bg-2: #1a2538;
        --panel: #151f31;
        --text: #e6edf7;
        --muted: #a3b2c7;
        --accent: #4f8bff;
        --accent-2: #2d6be4;
        --secondary: #45566f;
        --input-bg: #0f1727;
        --code-bg: #1f2a3f;
        --good: #46ba78;
        --warn: #ff8578;
        --border: #2f3d56;
        --card-bg: #11203d;
        --card-border: #38548f;
        --card-text: #a8c6ff;
        --shadow: 0 8px 24px rgba(0, 0, 0, 0.35);
        --pill-host-bg: #20345f;
        --pill-host-text: #a5c0ff;
        --pill-voted-bg: #203e31;
        --pill-value-bg: #47381d;
        --pill-value-text: #ffd58a;
        --theme-select-bg: #0f1727;
      }
    }
    * { box-sizing: border-box; }
    body {
      margin: 0;
      font-family: Segoe UI, system-ui, -apple-system, sans-serif;
      color: var(--text);
      background: radial-gradient(circle at top left, var(--bg-2), var(--bg) 40%);
      min-height: 100vh;
    }
    .wrap { max-width: 920px; margin: 0 auto; padding: 16px; }
    .panel {
      background: var(--panel);
      border: 1px solid var(--border);
      border-radius: 12px;
      padding: 16px;
      box-shadow: var(--shadow);
      margin-bottom: 16px;
    }
    h1 { margin: 0 0 8px; font-size: 1.7rem; }
    p { margin: 0; color: var(--muted); }
    .row { display: flex; gap: 8px; flex-wrap: wrap; }
    input[type="text"] {
      flex: 1;
      min-width: 220px;
      padding: 10px 12px;
      border: 1px solid var(--border);
      background: var(--input-bg);
      color: var(--text);
      border-radius: 8px;
      font-size: 1rem;
    }
    button {
      border: none;
      border-radius: 8px;
      padding: 10px 14px;
      font-size: 0.95rem;
      cursor: pointer;
      color: #fff;
      background: var(--accent);
    }
    button:hover { background: var(--accent-2); }
    button.secondary { background: var(--secondary); }
    button.secondary:hover { filter: brightness(1.08); }
    .theme-select {
      margin-left: auto;
      border: 1px solid var(--border);
      border-radius: 8px;
      background: var(--theme-select-bg);
      color: var(--text);
      padding: 7px 10px;
      font-size: 0.9rem;
    }
    .cards {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(72px, 1fr));
      gap: 8px;
      margin-top: 12px;
    }
    .card {
      background: var(--card-bg);
      border: 1px solid var(--card-border);
      color: var(--card-text);
      font-weight: 700;
      padding: 14px;
      border-radius: 10px;
      text-align: center;
      cursor: pointer;
      user-select: none;
    }
    .card.selected {
      background: #0b5fff;
      border-color: #0b5fff;
      color: #fff;
    }
    .pill {
      display: inline-block;
      border-radius: 999px;
      padding: 4px 10px;
      font-size: 0.8rem;
      font-weight: 600;
      margin-left: 8px;
    }
    .pill.host { background: var(--pill-host-bg); color: var(--pill-host-text); }
    .pill.voted { background: var(--pill-voted-bg); color: var(--good); }
    .pill.value { background: var(--pill-value-bg); color: var(--pill-value-text); }
    .participants { display: grid; gap: 8px; margin-top: 10px; }
    .participant {
      border: 1px solid var(--border);
      border-radius: 8px;
      padding: 10px;
      display: flex;
      justify-content: space-between;
      align-items: center;
    }
    .hidden { display: none; }
    .status { font-weight: 600; color: var(--muted); }
    .error { color: var(--warn); font-weight: 600; margin-top: 8px; }
    .topic { margin-top: 10px; color: var(--muted); }
    .topic strong { color: var(--text); }
    .summary { margin-top: 8px; color: var(--muted); font-size: 0.95rem; }
    .copy-row { display: flex; align-items: center; gap: 8px; margin-top: 6px; flex-wrap: wrap; }
    .copy-status { font-size: 0.85rem; color: var(--muted); }
    code { background: var(--code-bg); padding: 2px 6px; border-radius: 6px; }
  </style>
</head>
<body>
  <div class="wrap">
    <div class="panel">
      <div class="row" style="align-items:center;">
        <h1 style="margin:0;">StoryVote</h1>
        <select id="themeSelect" class="theme-select" aria-label="Theme">
          <option value="system">Theme: System</option>
          <option value="light">Theme: Light</option>
          <option value="dark">Theme: Dark</option>
        </select>
      </div>
      <p>Share this URL with your team: <code id="shareUrl"></code></p>
      <div class="copy-row">
        <button id="copyShareBtn" class="secondary" type="button">Copy Share URL</button>
        <span id="copyShareStatus" class="copy-status"></span>
      </div>
    </div>

    <div id="joinPanel" class="panel">
      <h2>Join Session</h2>
      <div class="row">
        <input id="nameInput" type="text" maxlength="32" placeholder="Enter your display name" />
        <button id="joinBtn">Join</button>
      </div>
      <div id="joinError" class="error hidden"></div>
    </div>

    <div id="appPanel" class="hidden">
      <div class="panel">
        <div class="row" style="justify-content:space-between;align-items:center;">
          <div>
            <strong id="meLabel"></strong>
            <span id="hostPill" class="pill host hidden">Host</span>
          </div>
          <div class="status" id="roundStatus">Waiting for votes</div>
        </div>

        <div class="topic">Estimating: <strong id="topicLabel">Not set</strong></div>
        <div id="summaryStatus" class="summary">Summary appears after reveal.</div>

        <div id="topicEditor" class="row hidden" style="margin-top:10px;">
          <input id="topicInput" type="text" maxlength="120" placeholder="What are we estimating?" />
          <button id="topicBtn" class="secondary">Set Topic</button>
        </div>

        <div class="cards" id="cards"></div>

        <div id="hostControls" class="row hidden" style="margin-top:12px;">
          <button id="revealBtn">Reveal</button>
          <button id="resetBtn" class="secondary">Reset</button>
        </div>
      </div>

      <div class="panel">
        <h2>Participants</h2>
        <div id="participants" class="participants"></div>
      </div>
    </div>
  </div>

  <script>
    const cards = ['0', '1', '2', '3', '5', '8', '13', '21', '34', '55', '?'];
    const THEME_KEY = 'storyVoteTheme';
    const state = {
      ws: null,
      sessionId: null,
      name: null,
      isHost: false,
      selectedVote: null,
      revealed: false,
      participants: [],
      votes: {},
      topic: '',
    };

    const shareUrlEl = document.getElementById('shareUrl');
    const copyShareBtn = document.getElementById('copyShareBtn');
    const copyShareStatus = document.getElementById('copyShareStatus');
    const themeSelect = document.getElementById('themeSelect');
    const joinPanel = document.getElementById('joinPanel');
    const appPanel = document.getElementById('appPanel');
    const joinError = document.getElementById('joinError');
    const nameInput = document.getElementById('nameInput');
    const joinBtn = document.getElementById('joinBtn');
    const meLabel = document.getElementById('meLabel');
    const hostPill = document.getElementById('hostPill');
    const roundStatus = document.getElementById('roundStatus');
    const hostControls = document.getElementById('hostControls');
    const topicEditor = document.getElementById('topicEditor');
    const topicInput = document.getElementById('topicInput');
    const topicBtn = document.getElementById('topicBtn');
    const topicLabel = document.getElementById('topicLabel');
    const summaryStatus = document.getElementById('summaryStatus');
    const participantsEl = document.getElementById('participants');
    const savedName = window.localStorage.getItem('storyVoteName');
    const savedTheme = window.localStorage.getItem(THEME_KEY);

    applyTheme(savedTheme || 'system');
    themeSelect.value = savedTheme || 'system';
    themeSelect.addEventListener('change', () => {
      applyTheme(themeSelect.value);
      window.localStorage.setItem(THEME_KEY, themeSelect.value);
    });

    shareUrlEl.textContent = window.location.href;
    copyShareBtn.addEventListener('click', copyShareUrl);
    if (savedName) {
      nameInput.value = savedName;
    }

    const cardsEl = document.getElementById('cards');
    cards.forEach((value) => {
      const card = document.createElement('div');
      card.className = 'card';
      card.textContent = value;
      card.addEventListener('click', () => castVote(value));
      cardsEl.appendChild(card);
    });

    joinBtn.addEventListener('click', joinSession);
    nameInput.addEventListener('keydown', (event) => {
      if (event.key === 'Enter') joinSession();
    });

    document.getElementById('revealBtn').addEventListener('click', () => send({ type: 'reveal' }));
    document.getElementById('resetBtn').addEventListener('click', () => {
      state.selectedVote = null;
      send({ type: 'reset' });
      renderCards();
    });

    topicBtn.addEventListener('click', setTopic);
    topicInput.addEventListener('keydown', (event) => {
      if (event.key === 'Enter') setTopic();
    });

    function setTopic() {
      send({ type: 'setTopic', value: topicInput.value });
    }

    function applyTheme(mode) {
      if (mode === 'light') {
        document.documentElement.setAttribute('data-theme', 'light');
        return;
      }

      if (mode === 'dark') {
        document.documentElement.setAttribute('data-theme', 'dark');
        return;
      }

      document.documentElement.removeAttribute('data-theme');
    }

    async function copyShareUrl() {
      const shareUrl = shareUrlEl.textContent.trim();
      if (!shareUrl) {
        copyShareStatus.textContent = 'No URL to copy yet.';
        return;
      }

      try {
        if (navigator.clipboard && typeof navigator.clipboard.writeText === 'function') {
          await navigator.clipboard.writeText(shareUrl);
        } else if (!copyTextWithExecCommand(shareUrl)) {
          throw new Error('clipboard unavailable');
        }
        copyShareStatus.textContent = 'Copied.';
      } catch (_) {
        if (copyTextWithExecCommand(shareUrl)) {
          copyShareStatus.textContent = 'Copied.';
        } else {
          copyShareStatus.textContent = 'Copy failed. Select and copy manually.';
        }
      }
    }

    function copyTextWithExecCommand(value) {
      const tempInput = document.createElement('textarea');
      tempInput.value = value;
      tempInput.setAttribute('readonly', '');
      tempInput.style.position = 'fixed';
      tempInput.style.top = '-9999px';
      tempInput.style.left = '-9999px';
      document.body.appendChild(tempInput);
      tempInput.focus();
      tempInput.select();

      let copied = false;
      try {
        copied = document.execCommand('copy');
      } catch (_) {
        copied = false;
      }

      document.body.removeChild(tempInput);
      return copied;
    }

    function joinSession() {
      const name = nameInput.value.trim();
      if (!name) {
        showJoinError('Please enter your name.');
        return;
      }
      showJoinError('');
      connect(name);
    }

    function connect(name) {
      const protocol = window.location.protocol === 'https:' ? 'wss' : 'ws';
      const url = `${protocol}://${window.location.host}/ws?name=${encodeURIComponent(name)}`;
      const ws = new WebSocket(url);
      joinBtn.disabled = true;

      ws.onopen = () => {
        state.ws = ws;
        state.name = name;
        window.localStorage.setItem('storyVoteName', name);
      };

      ws.onmessage = (event) => {
        const payload = JSON.parse(event.data);
        onServerEvent(payload);
      };

      ws.onerror = () => {
        showJoinError('Unable to connect to the session.');
      };

      ws.onclose = () => {
        const hadSession = Boolean(state.sessionId);
        state.ws = null;
        state.sessionId = null;
        state.isHost = false;
        joinBtn.disabled = false;
        hostPill.classList.add('hidden');
        hostControls.classList.add('hidden');
        topicEditor.classList.add('hidden');

        if (hadSession) {
          appPanel.classList.add('hidden');
          joinPanel.classList.remove('hidden');
          showJoinError('Disconnected. Join again to continue.');
        }
      };
    }

    function onServerEvent(payload) {
      if (payload.type === 'error') {
        if (!state.sessionId) {
          showJoinError(payload.message);
          joinBtn.disabled = false;
        } else {
          roundStatus.textContent = payload.message;
        }
        return;
      }

      if (payload.type === 'connected') {
        state.sessionId = payload.sessionId;
        state.isHost = payload.isHost;
        meLabel.textContent = payload.name;
        hostPill.classList.toggle('hidden', !payload.isHost);
        hostControls.classList.toggle('hidden', !payload.isHost);
        joinPanel.classList.add('hidden');
        appPanel.classList.remove('hidden');
        joinBtn.disabled = false;
        return;
      }

      if (payload.type === 'state') {
        state.revealed = payload.revealed;
        state.participants = payload.participants;
        state.votes = payload.votes || {};
        state.topic = payload.topic || '';
        state.isHost = state.participants.some((participant) => participant.name === state.name && participant.isHost);
        hostPill.classList.toggle('hidden', !state.isHost);
        hostControls.classList.toggle('hidden', !state.isHost);
        topicEditor.classList.toggle('hidden', !state.isHost);
        renderState();
      }
    }

    function castVote(value) {
      state.selectedVote = value;
      renderCards();
      send({ type: 'vote', value });
    }

    function send(payload) {
      if (!state.ws || state.ws.readyState !== WebSocket.OPEN) return;
      state.ws.send(JSON.stringify(payload));
    }

    function renderState() {
      roundStatus.textContent = state.revealed ? 'Votes revealed' : 'Waiting for votes';
      topicLabel.textContent = state.topic || 'Not set';
      if (state.isHost && topicInput.value !== state.topic) {
        topicInput.value = state.topic;
      }
      renderSummary();
      renderParticipants();
      renderCards();
    }

    function renderSummary() {
      if (!state.revealed) {
        summaryStatus.textContent = 'Summary appears after reveal.';
        return;
      }

      const totalCount = Object.keys(state.votes).length;
      if (totalCount === 0) {
        summaryStatus.textContent = 'No votes submitted.';
        return;
      }

      const numericVotes = Object.values(state.votes)
        .map((value) => Number(value))
        .filter((value) => Number.isFinite(value))
        .sort((a, b) => a - b);

      if (numericVotes.length === 0) {
        summaryStatus.textContent = 'No numeric votes to summarize.';
        return;
      }

      const sum = numericVotes.reduce((acc, value) => acc + value, 0);
      const average = sum / numericVotes.length;
      const middle = Math.floor(numericVotes.length / 2);
      const median = numericVotes.length % 2 === 0
        ? (numericVotes[middle - 1] + numericVotes[middle]) / 2
        : numericVotes[middle];

      const excludedCount = totalCount - numericVotes.length;
      const averageText = Number.isInteger(average) ? String(average) : average.toFixed(2).replace(/\.00$/, '');
      const medianText = Number.isInteger(median) ? String(median) : median.toFixed(2).replace(/\.00$/, '');

      summaryStatus.textContent = `Summary: Avg ${averageText} | Median ${medianText} | Numeric ${numericVotes.length}/${totalCount}`
        + (excludedCount > 0 ? ` (${excludedCount} non-numeric excluded)` : '');
    }

    function renderCards() {
      Array.from(cardsEl.children).forEach((card) => {
        card.classList.toggle('selected', card.textContent === state.selectedVote);
      });
    }

    function renderParticipants() {
      participantsEl.innerHTML = '';
      state.participants
        .slice()
        .sort((a, b) => a.name.localeCompare(b.name))
        .forEach((participant) => {
          const row = document.createElement('div');
          row.className = 'participant';
          const left = document.createElement('div');
          left.textContent = participant.name;

          const right = document.createElement('div');
          if (participant.isHost) {
            const host = document.createElement('span');
            host.className = 'pill host';
            host.textContent = 'Host';
            right.appendChild(host);
          }
          if (participant.voted) {
            const voted = document.createElement('span');
            voted.className = 'pill voted';
            voted.textContent = 'Voted';
            right.appendChild(voted);
          }
          if (state.revealed && Object.prototype.hasOwnProperty.call(state.votes, participant.name)) {
            const value = document.createElement('span');
            value.className = 'pill value';
            value.textContent = state.votes[participant.name];
            right.appendChild(value);
          }

          row.appendChild(left);
          row.appendChild(right);
          participantsEl.appendChild(row);
        });
    }

    function showJoinError(message) {
      if (!message) {
        joinError.textContent = '';
        joinError.classList.add('hidden');
        return;
      }
      joinError.textContent = message;
      joinError.classList.remove('hidden');
    }
  </script>
</body>
</html>
"#
}

#[cfg(test)]
mod tests {
  use super::index_html;

  #[test]
  fn ui_html_contains_key_elements() {
    let html = index_html();

    assert!(html.contains("id=\"shareUrl\""));
    assert!(html.contains("id=\"joinBtn\""));
    assert!(html.contains("id=\"nameInput\""));
    assert!(html.contains("id=\"appPanel\""));
    assert!(html.contains("id=\"themeSelect\""));
    assert!(html.contains("id=\"copyShareBtn\""));
    assert!(html.contains("id=\"summaryStatus\""));
    assert!(html.contains("/ws?name=${encodeURIComponent(name)}"));
  }

  #[test]
  fn ui_html_does_not_contain_escaped_quotes() {
    let html = index_html();

    assert!(
      !html.contains("\\\""),
      "ui template contains escaped quotes that can break rendered HTML"
    );
  }
}
