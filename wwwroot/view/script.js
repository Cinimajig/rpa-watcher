const intervalSeconds = 5;
const rpaView = document.querySelector('#rpa-view');
const rpaRuns = document.querySelector('#rpa-runs');
const rpaHistoryView = document.querySelector('#rpa-history');
const historyViewContainer = document.querySelector('#history-view');
const info = document.querySelector('.no-info');

let getRpaDataLink = '/api/getrpa';
let getHistoryRpaDataLink = '/api/gethistory?amount=50';

const subflow = `<img src="Down_Right.svg" alt="Child flow" class="image" />`;
const defaultLogo = `<img src="parent.svg" alt="Unknown engine or child flow" class="image" />`;
let paLogo = `<img src="PALogo.png" alt="Power Automate" class="image" />`;
let prLogo = `<img src="PRLogo.png" alt="ProcessRobot" class="image" />`;
let timeZone = null;
let noBlue = false;
let noRed = false;

const parseBool = (str) => {
    return str.trim().toLowerCase() == 'true'
}

for (let param of window.location.search.substring(1).split('&')) {
    const [name, value] = param.split('=');

    switch (name) {
        case 'tz':
            timeZone = value.trim();
            break;
        case 'no-color':
            if (value ? parseBool(value) : true) {
                prLogo = `<img src="PRLogo_GS.png" alt="ProcessRobot" class="image" />`;
                paLogo = `<img src="PALogo_GS.png" alt="Power Automate" class="image" />`;
                noRed = true;
                noBlue = true;
            }
            break;
        case 'no-red':
            if (value ? parseBool(value) : true) {
                prLogo = `<img src="PRLogo_GS.png" alt="ProcessRobot" class="image" />`;
                noRed = true;
            }
            break;
        case 'no-blue':
            if (value ? parseBool(value) : true) {
                paLogo = `<img src="PALogo_GS.png" alt="Power Automate" class="image" />`;
                noBlue = true;
            }
            break;
    }
}

let rpaData = new Map();
let historyRpaData = new Map();

const parseTrigger = (str) => {
    const prefix = 'Started from Console by';
    if (str.startsWith(prefix)) {
        return str.slice(prefix.length).trim();
    }
    return str;
}

const findParent = (str) => {
    if (str) {
        for (let val of rpaData.entries()) {
            if (val[1].instance === str) {
                return val[1].name;
            }
        }
    }

    return str;
}

const buildRpaRuns = async (clear) => {
    let data = await getRpaData();

    if (clear) {
        rpaData.clear();
    }

    for (let i = 0; i < data.length; i++) {
        rpaData.set(data[i].instance, data[i]);
    }

    for (let el of rpaRuns.querySelectorAll('.process')) {
        let attr = el.getAttribute('data-ref');
        if (!iteratorIncludes(attr, rpaData.keys())) {
            let item = rpaRuns.querySelector(`.process[data-ref="${attr}"]`);
            if (item) { item.remove(); }
        }
    }

    // OLD: a[1].parentInstance !== null
    appendItemsEx(rpaRuns, [...rpaData.entries()].sort((a, b) => a[1].started > b[1].started ? 1 : -1), false);

    if (rpaData.size === 0) {
        info.style.display = '';
    } else {
        info.style.display = 'none';
    }

    await buildHistory(clear);
}

const appendItemsEx = (root, items, noParent) => {
    // Changes the current action, in case it has changed.
    for (let rpa of items) {
        const currentFlow = document.querySelector(`.process[data-ref="${rpa[0]}"]`);
        if (currentFlow) {
            if (!rpa[1].action && !rpa[1].notification) {
                continue;
            }

            const errorBlock = currentFlow.querySelector('.action .actionerrblock');
            const actionFunc = currentFlow.querySelector('.action .actionfunc');
            const actionIndex = currentFlow.querySelector('.action .actionindex');
            const actionName = currentFlow.querySelector('.action .actionname');

            // Notification.
            const notification = currentFlow.querySelector('.notification');
            if (rpa[1].notification) {
                notification.innerText = rpa[1].notification;
            } else {
                notification.innerText = '';
            }

            // Error block.
            rpa[1].action.insideErrorHandling ? errorBlock.classList.add('shield') : errorBlock.classList.remove('shield');

            // Function name.
            actionFunc.innerText = rpa[1].action.functionName;

            // Action index.
            actionIndex.innerText = rpa[1].action.index;

            // Locale action name.
            actionName.innerText = rpa[1].action.name;

            continue;
        }

        const template = document.querySelector('template.rpa-run').content.cloneNode(true);
        const newRunItem = template.querySelector('.rpa-run.process');

        newRunItem.setAttribute('data-ref', rpa[0]);
        // .head
        const head = newRunItem.querySelector('.head.row');
        const hostname = newRunItem.querySelector('.item.hostname');
        const flowName = newRunItem.querySelector('.item.name');
        // .body
        const engine = newRunItem.querySelector('.item.engine');
        const trigger = newRunItem.querySelector('.item.trigger');
        const started = newRunItem.querySelector('.item.started');
        // const parent = newRunItem.querySelector('.item.parent');

        // .action
        const action = newRunItem.querySelector('.action.row');
        const errorBlock = newRunItem.querySelector('.item.actionerrblock');
        const actionFunc = newRunItem.querySelector('.item.actionfunc');
        const actionIndex = newRunItem.querySelector('.item.actionindex');
        const actionName = newRunItem.querySelector('.item.actionname');
        
        const notification = newRunItem.querySelector('.notification');

        switch (rpa[1].engine) {
            case 'Power Automate':
                if (!noBlue) head.classList.add('pad');
                engine.innerHTML = paLogo;
                break;
            case 'ProcessRobot':
                if (!noRed) head.classList.add('pr');
                engine.innerHTML = prLogo;
                break;
            default:
                engine.innerHTML = defaultLogo;
                break;
        }

        hostname.innerText = rpa[1].computer.trim();
        trigger.innerText = rpa[1].trigger ? parseTrigger(rpa[1].trigger.trim()) : '';
        flowName.innerText = rpa[1].name ? rpa[1].name.trim() : rpa[1].instance.trim();
        try {
            const dt = new Date(rpa[1].started.trim());
            if (timeZone) {
                started.innerText = dt.toLocaleString(timeZone);
            } else {
                started.innerText = dt.toLocaleString();
            }
        } catch {
            started.innerText = '';
        }

        // Notification.
        if (rpa[1].notification) {
            notification.innerText = rpa[1].notification;
        } else {
            notification.innerText = '';
        }
        
        if (rpa[1].action) {
            rpa[1].action.insideErrorHandling ? errorBlock.classList.add('shield') : errorBlock.classList.remove('shield');
            actionFunc.innerText = rpa[1].action.functionName;
            actionIndex.innerText = rpa[1].action.index;
            actionName.innerText = rpa[1].action.name;
            action.style.display = '';
        } else {
            action.style.display = 'none';
        }
        
        if (!noParent && rpa[1].parentInstance) {
            // parent.innerText = findParent(rpa[1].parentInstance);
            let parentElement = document.querySelector(`.process[data-ref="${rpa[1].parentInstance}"]`);
            
            // engine.innerHTML = defaultLogo;
            // if (parentElement && parentElement.nextSibling) {
            //     root.insertBefore(newRunItem, parentElement.nextSibling);
            //     continue;
            // }
            if (parentElement) {
                head.remove();
                started.remove();

                newRunItem.style.boxShadow = 'none';
                newRunItem.style.marginBottom = '';
                engine.innerHTML = subflow;
                trigger.innerText = flowName.innerText;
                trigger.className = 'item child';
                parentElement.appendChild(newRunItem);
                continue;
            }
        }

        root.appendChild(newRunItem);
    }
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
            let item = rpaView.querySelector(`.tr.rpa-info[data-ref="${attr}"]`);
            if (item) { item.remove(); }
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
            let item = rpaHistoryView.querySelector(`.tr.rpa-info[data-ref="${attr}"]`);
            if (item) { item.remove(); }
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
        const flowName = tr.querySelector('.td.name');
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
        trigger.innerText = rpa[1].trigger ? parseTrigger(rpa[1].trigger.trim()) : '';
        flowName.innerText = rpa[1].name ? rpa[1].name.trim() : rpa[1].instance.trim();
        try {
            started.innerText = new Date(rpa[1].started.trim()).toLocaleString();
        } catch {
            started.innerText = '';
        }

        if (rpa[1].notification) {
            parent.innerText = rpa[1].notification.trim();
        }

        // if (!noParent && rpa[1].parentInstance) {
        //     parent.innerText = rpa[1].parentInstance?.trim();
        //     let parentElement = document.querySelector(`.tr.rpa-info[data-ref="${rpa[1].parentInstance}"`);

        //     engine.innerHTML = defaultLogo;
        //     if (parentElement && parentElement.nextSibling) {
        //         root.insertBefore(tr, parentElement.nextSibling);
        //         continue;
        //     }
        // }

        root.appendChild(tr);
    }
}

const getRpaRunIds = () => rpaData.keys();
const getHistoryRpaData = async () => (await fetch(getHistoryRpaDataLink)).json()
const getRpaData = async () => (await fetch(getRpaDataLink)).json();

const clearCanvasEx = () => {
    rpaData.clear();
    historyRpaData.clear();
    rpaRuns.innerHTML = '';
}

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
    buildRpaRuns(true).catch((err) => {
        clearCanvasEx();
        console.error(err);
    })
}, intervalSeconds * 1000);
buildRpaRuns(false);

globalThis.clearTimer = () => {
    clearInterval(timer);
}

globalThis.testdata = () => {
    getRpaDataLink = '/api/gettemplate';
    getHistoryRpaDataLink = '/api/gethistorytemplate';
    clearInterval(timer);
    
    buildRpaRuns(true).catch((err) => {
        clearCanvasEx();
        console.error(err);
    })
}