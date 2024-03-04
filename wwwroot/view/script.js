const timeoutSeconds = 7;
const rpaView = document.querySelector('#rpa-view');
const info = document.querySelector('.no-info');

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

    for (let rpa of rpaData.entries()) {
        info.style.display = '';
        if (document.querySelector(`.tr.rpa-info[data-ref="${rpa[0]}"`)) {
            continue;
        }

        const template = document.querySelector('template.rpa-info').content.cloneNode(true);
        const tr = template.querySelector('.tr.rpa-info');

        tr.setAttribute('data-ref', rpa[0]);
        const engine = tr.querySelector('.td.engine');
        const hostname = tr.querySelector('.td.hostname');
        const env = tr.querySelector('.td.env');
        const instance = tr.querySelector('.td.instance');
        const flowId = tr.querySelector('.td.name');
        const teanantId = tr.querySelector('.td.extra');


        switch (rpa[1].engine) {
            case 'Power Automate':
                engine.classList.add('pad');
                break;
            case 'ProcessRobot':
                engine.classList.add('pr');
                break;
        }

        engine.innerText = rpa[1].engine.trim();
        hostname.innerText = rpa[1].computer.trim();
        env.innerText = rpa[1].env ? rpa[1].env : 'null...';
        instance.innerText = rpa[1].instance.trim();

        if (rpa[1]?.azureData) {
            flowId.innerText = rpa[1].azureData.flowId.trim();
            teanantId.innerText = rpa[1].azureData.tenantId.trim();

            if (engine.classList.contains('pad') && rpa[1].env) {
                instance.innerHTML = `<a target="_blank" href="https://make.powerautomate.com/environments/${env.innerText}/uiflows/${flowId.innerText}/runs/${instance.innerText}">${instance.innerText}</a>`;
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
const getFailedRpaData = async () => {}

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
}, timeoutSeconds * 1000);
buildRpaConvas();

globalThis.clearTimer = (really) => {
    if (really === 'Really') {
        clearInterval(timer);
    }
}

globalThis.insertTestData = (times) => {
    for (let i = 0; i < times; i++) {
        rpaData.set('b415296d-aea8-48d9-aea9-053d77450f2b' + i, {
            engine: 'Power Automate',
            computer: 'TESTMACHINE',
            env: 'b415296d-aea8-48d9-aea9-053d77450f2b',
            instance: 'b415296d-aea8-48d9-aea9-053d77450f2b',
            azureData: {
                flowId: 'b415296d-aea8-48d9-aea9-053d77450f2b',
                tenantId: 'b415296d-aea8-48d9-aea9-053d77450f2b'
            }
        });
    }

    buildRpaConvas(false);
}