const intervalSeconds = 5;
const rpaView = document.querySelector('#rpa-view');
const info = document.querySelector('.no-info');

if (window.location.search.includes('word-brake')) {
    rpaView.classList.add('wordbreak');
}

let rpaData = new Map();
let failedRpaData = new Map();

const buildRpaConvas = async (clear) => {
    let data = await getRpaData();
    let failed = await getFailedRpaData();

    if (clear) {
        rpaData.clear();
    }
    for (let i = 0; i < data.length; i++) {
        rpaData.set(data[i].instance, data[i]);
    }

    for (let el of document.querySelectorAll('.tr.rpa-info')) {
        let attr = el.getAttribute('data-ref');
        if (!iteratorIncludes(attr, rpaData.keys())) {
            document.querySelector(`.tr.rpa-info[data-ref="${attr}"`).remove();
        }
    }

    for (let rpa of [...rpaData.entries()].sort((a, b) => a[1].parentInstance !== null)) {
        info.style.display = '';
        if (document.querySelector(`.tr.rpa-info[data-ref="${rpa[0]}"`)) {
            continue;
        }

        const template = document.querySelector('template.rpa-info').content.cloneNode(true);
        const tr = template.querySelector('.tr.rpa-info');

        tr.setAttribute('data-ref', rpa[0]);
        const engine = tr.querySelector('.td.engine');
        const hostname = tr.querySelector('.td.hostname');
        const trigger = tr.querySelector('.td.trigger');
        const instance = tr.querySelector('.td.instance');
        const flowId = tr.querySelector('.td.name');
        const parent = tr.querySelector('.td.parent');

        switch (rpa[1].engine) {
            case 'Power Automate':
                // engine.classList.add('pad');
                engine.innerHTML = `<img src="PALogo.png" alt="Power Automate" class="image" />`;
                break;
            case 'ProcessRobot':
                // engine.classList.add('pr');
                engine.innerHTML = `<img src="PRLogo.png" alt="ProcessRobot" class="image" />`;
                break;
            default:
                engine.innerHTML = `<img src="parent.svg" alt="Unknown engine" class="image" />`;
                break;
        }

        // engine.innerText = rpa[1].engine.trim();
        hostname.innerText = rpa[1].computer.trim();
        trigger.innerText = rpa[1].trigger ? rpa[1].trigger.trim() : '';
        instance.innerText = rpa[1].instance.trim();
        flowId.innerText = rpa[1].flowId ? rpa[1].flowId.trim() : '';

        if (rpa[1].parentInstance) {
            parent.innerText = rpa[1].parentInstance?.trim();
            let parentElement = document.querySelector(`.tr.rpa-info[data-ref="${rpa[1].parentInstance}"`);

            if (parentElement && parentElement.nextSibling) {
                engine.innerHTML = `<img src="parent.svg" alt="Child of a flow" class="image" />`;
                rpaView.insertBefore(tr, parentElement.nextSibling);
                continue;
            }
        }

        rpaView.appendChild(tr);
    }

    if (rpaData.size === 0) {
        info.style.display = '';
    } else {
        info.style.display = 'none';
    }
}

const getRpaRunIds = () => rpaData.keys();

const getRpaData = async () => (await fetch('/api/getrpa')).json();
const getFailedRpaData = async () => { }

const clearCanvas = () => {
    rpaData.clear();
    failedRpaData.clear();
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
buildRpaConvas();

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
