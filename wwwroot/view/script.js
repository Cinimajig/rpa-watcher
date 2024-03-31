const intervalSeconds = 5;
const rpaView = document.querySelector('#rpa-view');
const rpaHistoryView = document.querySelector('#rpa-history');
const historyViewContainer = document.querySelector('#history-view');
const info = document.querySelector('.no-info');

const defaultLogo = `<img src="parent.svg" alt="Unknown engine or child flow" class="image" />`;
const paLogo = `<img src="PALogo.png" alt="Power Automate" class="image" />`;
const prLogo = `<img src="PRLogo.png" alt="ProcessRobot" class="image" />`;

if (window.location.search.includes('word-break')) {
    rpaView.classList.add('wordbreak');
}

let rpaData = new Map();
let historyRpaData = new Map();

const parse_trigger = (str) => {
    if (str.startsWith('Started from Console by')) {
        return str.slice(23).trim();
    }
    return str;
}

const buildRpaConvas = async (clear) => {
    let data = await getRpaData();

    if (clear) {
        rpaData.clear();
    }

    for (let i = 0; i < data.length; i++) {
        rpaData.set(data[i].instance, data[i]);
    }

    for (let el of rpaView.querySelectorAll('.tr.rpa-info')) {
        let attr = el.getAttribute('data-ref');
        if (!iteratorIncludes(attr, rpaData.keys())) {
            rpaView.querySelector(`.tr.rpa-info[data-ref="${attr}"`).remove();
        }
    }

    appendItems(rpaView, [...rpaData.entries()].sort((a, b) => a[1].parentInstance !== null), false);

    if (rpaData.size === 0) {
        info.style.display = '';
    } else {
        info.style.display = 'none';
    }

    await buildHistory(clear);
}

const buildHistory = async (clear) => {
    let history = await getHistoryRpaData();

    if (clear) {
        historyRpaData.clear();
    }

    for (let i = 0; i < history.length; i++) {
        historyRpaData.set(history[i].instance, history[i]);
    }

    for (let el of rpaHistoryView.querySelectorAll('.tr.rpa-info')) {
        let attr = el.getAttribute('data-ref');
        if (!iteratorIncludes(attr, historyRpaData.keys())) {
            rpaHistoryView.querySelector(`.tr.rpa-info[data-ref="${attr}"`).remove();
        }
    }

    appendItems(rpaHistoryView, historyRpaData, true);
    
    if (historyRpaData.size === 0) {
        historyViewContainer.style.display = 'none';
    } else {
        historyViewContainer.style.display = '';
    }
}

const appendItems = (root, items, noParent) => {
    for (let rpa of items) {
        info.style.display = '';
        if (root.querySelector(`.tr.rpa-info[data-ref="${rpa[0]}"`)) {
            continue;
        }

        const template = document.querySelector('template.rpa-info').content.cloneNode(true);
        const tr = template.querySelector('.tr.rpa-info');

        tr.setAttribute('data-ref', rpa[0]);
        const engine = tr.querySelector('.td.engine');
        const hostname = tr.querySelector('.td.hostname');
        const trigger = tr.querySelector('.td.trigger');
        const started = tr.querySelector('.td.started');
        const flowId = tr.querySelector('.td.name');
        const parent = tr.querySelector('.td.parent');

        switch (rpa[1].engine) {
            case 'Power Automate':
                // engine.classList.add('pad');
                engine.innerHTML = paLogo;
                break;
            case 'ProcessRobot':
                // engine.classList.add('pr');
                engine.innerHTML = prLogo;
                break;
            default:
                engine.innerHTML = defaultLogo;
                break;
        }

        hostname.innerText = rpa[1].computer.trim();
        trigger.innerText = rpa[1].trigger ? parse_trigger(rpa[1].trigger.trim()) : '';
        flowId.innerText = rpa[1].flowId ? rpa[1].flowId.trim() : rpa[1].instance.trim();
        try {
            started.innerText = new Date(rpa[1].started.trim()).toLocaleString();
        } catch {
            started.innerText = '';
        }

        if (!noParent && rpa[1].parentInstance) {
            parent.innerText = rpa[1].parentInstance?.trim();
            let parentElement = document.querySelector(`.tr.rpa-info[data-ref="${rpa[1].parentInstance}"`);

            engine.innerHTML = defaultLogo;
            if (parentElement && parentElement.nextSibling) {
                root.insertBefore(tr, parentElement.nextSibling);
                continue;
            }
        }

        root.appendChild(tr);
    }
}

const getRpaRunIds = () => rpaData.keys();
const getRpaData = async () => (await fetch('/api/getrpa')).json();
const getHistoryRpaData = async () => (await fetch('/api/gethistory?amount=50')).json()

const clearCanvas = () => {
    rpaData.clear();
    historyRpaData.clear();
    rpaView.innerHTML = '';
}

const iteratorIncludes = (item, iter) => {
    for (let el in iter) {
        if (el === item) return true
    }

    return false;
}

const timer = setInterval(() => {
    buildRpaConvas(true).catch((err) => {
        clearCanvas();
        console.error(err);
    })
}, intervalSeconds * 1000);
buildRpaConvas(false);

globalThis.clearTimer = (really) => {
    if (really === 'Really') {
        clearInterval(timer);
    }
}

globalThis.insertTestData = (times) => {
    rpaData.set('b415296d-aea8-48d9-aea9-053d77450f2b', {
        engine: 'Power Automate',
        computer: 'TESTMACHINE',
        trigger: 'Unattended',
        instance: 'b415296d-aea8-48d9-aea9-053d77450f2b',
        flowId: 'b415296d-aea8-48d9-aea9-053d77450f2b',
        parentInstance: null
    });
    rpaData.set('b415296d-aea8-48d9-aea9-053d77450f2c', {
        engine: 'ProcessRobot',
        computer: 'TESTMACHINE',
        trigger: 'Unattended',
        instance: 'b415296d-aea8-48d9-aea9-053d77450f2c',
        flowId: 'b415296d-aea8-48d9-aea9-053d77450f2c',
        parentInstance: 'b415296d-aea8-48d9-aea9-053d77450f2b'
    });
    rpaData.set('b415296d-aea8-48d9-aea9-053d77450f2d', {
        engine: 'ProcessRobot',
        computer: 'TESTMACHINE',
        trigger: 'Unattended',
        instance: 'b415296d-aea8-48d9-aea9-053d77450f2c',
        flowId: 'b415296d-aea8-48d9-aea9-053d77450f2c',
        parentInstance: null
    });

    buildRpaConvas(false);
}
