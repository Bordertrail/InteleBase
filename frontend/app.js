// InteleBase Frontend - Vanilla JavaScript

const API_BASE = '/api/v1';
let token = localStorage.getItem('intelebase_token');
let user = JSON.parse(localStorage.getItem('intelebase_user') || 'null');
let currentPage = 1;
let totalPages = 1;
let currentKbId = null;
let isLoginMode = true;

// Format datetime to yyyy-mm-dd HH:mm:ss
function formatDateTime(dateStr) {
    if (!dateStr) return '';
    const d = new Date(dateStr);
    const pad = n => n.toString().padStart(2, '0');
    return `${d.getFullYear()}-${pad(d.getMonth()+1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}:${pad(d.getSeconds())}`;
}

// DOM Elements
const loginPage = document.getElementById('login-page');
const dashboardPage = document.getElementById('dashboard-page');
const kbDetailPage = document.getElementById('kb-detail-page');
const loginTitle = document.getElementById('login-title');
const registerFields = document.getElementById('register-fields');
const authForm = document.getElementById('auth-form');
const submitBtn = document.getElementById('submit-btn');
const toggleModeBtn = document.getElementById('toggle-mode');
const errorMsg = document.getElementById('error-msg');
const userName = document.getElementById('user-name');
const userNameDetail = document.getElementById('user-name-detail');
const logoutBtn = document.getElementById('logout-btn');
const kbList = document.getElementById('kb-list');
const loading = document.getElementById('loading');
const pagination = document.getElementById('pagination');
const createKbBtn = document.getElementById('create-kb-btn');
const createModal = document.getElementById('create-modal');
const closeModalBtn = document.getElementById('close-modal');
const createKbForm = document.getElementById('create-kb-form');
const backToListBtn = document.getElementById('back-to-list');
const kbTitle = document.getElementById('kb-title');
const kbDescription = document.getElementById('kb-description');
const membersList = document.getElementById('members-list');
const noMembers = document.getElementById('no-members');
const addMemberBtn = document.getElementById('add-member-btn');
const addMemberModal = document.getElementById('add-member-modal');
const closeMemberModalBtn = document.getElementById('close-member-modal');
const addMemberForm = document.getElementById('add-member-form');

// API Functions
async function api(method, path, body = null) {
    const options = {
        method,
        headers: {
            'Content-Type': 'application/json',
        },
    };
    if (token) {
        options.headers['Authorization'] = `Bearer ${token}`;
    }
    if (body) {
        options.body = JSON.stringify(body);
    }
    const response = await fetch(`${API_BASE}${path}`, options);
    if (!response.ok) {
        // Token expired or invalid - clear cache and redirect to login
        if (response.status === 401) {
            logout();
            throw new Error('登录已过期，请重新登录');
        }
        const text = await response.text();
        throw new Error(`HTTP ${response.status}: ${text}`);
    }
    if (response.status === 204) return null;
    return response.json();
}

// Auth Functions
function showError(msg) {
    errorMsg.textContent = msg;
    errorMsg.classList.remove('hidden');
}

function hideError() {
    errorMsg.classList.add('hidden');
}

function toggleMode() {
    isLoginMode = !isLoginMode;
    loginTitle.textContent = isLoginMode ? '登录' : '注册';
    submitBtn.textContent = isLoginMode ? '登录' : '注册';
    toggleModeBtn.textContent = isLoginMode ? '没有账号？注册' : '已有账号？登录';
    registerFields.classList.toggle('hidden', isLoginMode);
    hideError();
}

async function handleAuth(e) {
    e.preventDefault();
    hideError();
    submitBtn.disabled = true;
    submitBtn.textContent = '处理中...';

    const email = document.getElementById('email').value;
    const password = document.getElementById('password').value;

    try {
        if (isLoginMode) {
            const resp = await api('POST', '/auth/login', { email, password });
            token = resp.access_token;
            user = resp.user;
            localStorage.setItem('intelebase_token', token);
            localStorage.setItem('intelebase_user', JSON.stringify(user));
            showDashboard();
        } else {
            const username = document.getElementById('username').value;
            const full_name = document.getElementById('full_name').value;
            await api('POST', '/auth/register', {
                username,
                email,
                password,
                full_name: full_name || null
            });
            showError('注册成功，请登录');
            toggleMode();
        }
    } catch (err) {
        showError(err.message);
    }

    submitBtn.disabled = false;
    submitBtn.textContent = isLoginMode ? '登录' : '注册';
}

function logout() {
    token = null;
    user = null;
    localStorage.removeItem('intelebase_token');
    localStorage.removeItem('intelebase_user');
    showLogin();
}

// Navigation
function showLogin() {
    loginPage.classList.remove('hidden');
    dashboardPage.classList.add('hidden');
    kbDetailPage.classList.add('hidden');
}

function showDashboard() {
    loginPage.classList.add('hidden');
    dashboardPage.classList.remove('hidden');
    kbDetailPage.classList.add('hidden');
    userName.textContent = user?.username || '';
    loadKbs();
}

function showKbDetail(kb) {
    loginPage.classList.add('hidden');
    dashboardPage.classList.add('hidden');
    kbDetailPage.classList.remove('hidden');
    userNameDetail.textContent = user?.username || '';
    currentKbId = kb.id;
    kbTitle.textContent = kb.name;
    kbDescription.textContent = kb.description || '';
    loadMembers();
}

// KB Functions
async function loadKbs() {
    loading.classList.remove('hidden');
    kbList.innerHTML = '';

    try {
        const result = await api('GET', `/knowledge-bases?page=${currentPage}&per_page=10`);
        totalPages = result.total_pages;

        if (result.items.length === 0) {
            kbList.innerHTML = '<div class="no-data">暂无知识库，点击上方按钮创建</div>';
        } else {
            result.items.forEach(kb => {
                const card = document.createElement('div');
                card.className = 'kb-card';
                card.innerHTML = `
                    <h3 class="kb-name">${kb.name}</h3>
                    <p class="kb-desc">${kb.description || ''}</p>
                    <span class="kb-date">创建于 ${formatDateTime(kb.created_at)}</span>
                    <div class="kb-actions">
                        <button class="kb-btn kb-btn-view" onclick="showKbDetail({id: ${kb.id}, name: '${kb.name}', description: '${kb.description || ''}'})">查看</button>
                        <button class="kb-btn kb-btn-delete" onclick="deleteKb(${kb.id})">删除</button>
                    </div>
                `;
                kbList.appendChild(card);
            });
        }

        renderPagination();
    } catch (err) {
        kbList.innerHTML = `<div class="no-data" style="color:#e53935">${err.message}</div>`;
    }

    loading.classList.add('hidden');
}

function renderPagination() {
    pagination.innerHTML = '';

    const prevBtn = document.createElement('button');
    prevBtn.className = 'px-4 py-2 bg-white rounded hover:bg-gray-100 disabled:bg-gray-300';
    prevBtn.textContent = '上一页';
    prevBtn.disabled = currentPage <= 1;
    prevBtn.onclick = () => { currentPage--; loadKbs(); };
    pagination.appendChild(prevBtn);

    const pageSpan = document.createElement('span');
    pageSpan.className = 'py-2 px-4 bg-white rounded';
    pageSpan.textContent = `${currentPage} / ${totalPages}`;
    pagination.appendChild(pageSpan);

    const nextBtn = document.createElement('button');
    nextBtn.className = 'px-4 py-2 bg-white rounded hover:bg-gray-100 disabled:bg-gray-300';
    nextBtn.textContent = '下一页';
    nextBtn.disabled = currentPage >= totalPages;
    nextBtn.onclick = () => { currentPage++; loadKbs(); };
    pagination.appendChild(nextBtn);
}

async function deleteKb(id) {
    if (!confirm('确定删除此知识库？')) return;
    try {
        await api('DELETE', `/knowledge-bases/${id}`);
        loadKbs();
    } catch (err) {
        alert(err.message);
    }
}

async function createKb(e) {
    e.preventDefault();
    const name = document.getElementById('kb-name').value;
    const description = document.getElementById('kb-desc').value;

    try {
        await api('POST', '/knowledge-bases', { name, description: description || null });
        createModal.classList.add('hidden');
        document.getElementById('kb-name').value = '';
        document.getElementById('kb-desc').value = '';
        loadKbs();
    } catch (err) {
        alert(err.message);
    }
}

// Member Functions
async function loadMembers() {
    membersList.innerHTML = '';
    noMembers.classList.add('hidden');

    try {
        const members = await api('GET', `/knowledge-bases/${currentKbId}/members`);
        if (members.length === 0) {
            noMembers.classList.remove('hidden');
        } else {
            members.forEach(m => {
                const row = document.createElement('tr');
                row.innerHTML = `
                    <td>${m.username}</td>
                    <td>${m.email}</td>
                    <td>${m.role_name}</td>
                    <td>
                        <button class="remove-btn" onclick="removeMember(${m.user_id})">移除</button>
                    </td>
                `;
                membersList.appendChild(row);
            });
        }
    } catch (err) {
        noMembers.textContent = err.message;
        noMembers.classList.remove('hidden');
    }
}

async function removeMember(userId) {
    if (!confirm('确定移除此成员？')) return;
    try {
        await api('DELETE', `/knowledge-bases/${currentKbId}/members/${userId}`);
        loadMembers();
    } catch (err) {
        alert(err.message);
    }
}

async function addMember(e) {
    e.preventDefault();
    const userId = parseInt(document.getElementById('member-user-id').value);
    const role = document.getElementById('member-role').value;

    try {
        await api('POST', `/knowledge-bases/${currentKbId}/members`, { user_id: userId, role });
        addMemberModal.classList.add('hidden');
        document.getElementById('member-user-id').value = '';
        loadMembers();
    } catch (err) {
        alert(err.message);
    }
}

// Initialize - verify token before showing dashboard
async function init() {
    if (token) {
        // Verify token is still valid
        try {
            await api('GET', '/knowledge-bases?page=1&per_page=1');
            showDashboard();
        } catch (err) {
            // Token invalid - already handled by api function (logout called)
            showLogin();
        }
    } else {
        showLogin();
    }
}

// Event Listeners
toggleModeBtn.addEventListener('click', toggleMode);
authForm.addEventListener('submit', handleAuth);
logoutBtn.addEventListener('click', logout);
createKbBtn.addEventListener('click', () => createModal.classList.remove('hidden'));
closeModalBtn.addEventListener('click', () => createModal.classList.add('hidden'));
createKbForm.addEventListener('submit', createKb);
backToListBtn.addEventListener('click', showDashboard);
addMemberBtn.addEventListener('click', () => addMemberModal.classList.remove('hidden'));
closeMemberModalBtn.addEventListener('click', () => addMemberModal.classList.add('hidden'));
addMemberForm.addEventListener('submit', addMember);

// Start
init();