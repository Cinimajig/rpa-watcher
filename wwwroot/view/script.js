const timeoutSeconds = 7;
const rpaView = document.querySelector('div#rpa-view');

let rpaData = new Map();
let failedRpaData = new Map();

const buildRpaConvas = async () => {
    let data = await getRpaData();
    let failed = await getFailedRpaData();

    rpaData.clear();
    for (let i = 0; i < data.length; i++) {
        rpaData.set(data[i].instance, data[i]);
    }

    for (let el of document.querySelectorAll('div.rpa-info')) {
        let attr = el.getAttribute('data-ref');
        if (!iteratorIncludes(attr, rpaData.keys())) {
            document.querySelector(`div.rpa-info[data-ref="${attr}"`).remove();
        }
    }

    for (let rpa of rpaData.entries()) {
        if (document.querySelector(`div.rpa-info[data-ref="${rpa[0]}"`)) {
            continue;
        }

        const template = document.querySelector('template.rpa-info').content.cloneNode(true);
        const div = template.querySelector('div.rpa-info');

        div.setAttribute('data-ref', rpa[0]);
        const engine = div.querySelector('div.engine');
        const hostname = div.querySelector('div.hostname');
        const env = div.querySelector('div.env');
        const instance = div.querySelector('div.instance');
        const flowId = div.querySelector('div.run-info div.name');
        const teanantId = div.querySelector('div.run-info div.extra');


        engine.innerText = rpa[1].engine;
        hostname.innerText = rpa[1].computer;
        env.innerText = rpa[1].env ? rpa[1].env : '';
        instance.innerText = rpa[1].instance;

        if (rpa[1]?.azureData) {
            flowId.innerText = rpa[1].azureData.flowId;
            teanantId.innerText = rpa[1].azureData.tenantId;
        }

        rpaView.appendChild(div);
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


setInterval(() => {
    buildRpaConvas().catch((err) => {
        clearCanvas();
        console.error(err);
    })
}, timeoutSeconds * 1000);
buildRpaConvas();
