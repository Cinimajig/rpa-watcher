const timeoutSeconds = 7;
const rpaView = document.querySelector('div#rpa-view');

let rpaData = new Map();
let failedRpaData = new Map();

setTimeout(() => {
    buildRpaConvas().catch((err) => {
        clearCanvas();
        console.error(err);
    })
}, timeoutSeconds * 1000);

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
        const template = document.querySelector('template.rpa-info').content.cloneNode(true);
        const div = template.querySelector('div.rpa-info');

        div.setAttribute('data-ref', rpa[0]);
        const engine = div.querySelector('div.engine');
        const hostname = div.querySelector('div.hostname');
        const env = div.querySelector('div.env');
        const instance = div.querySelector('div.instance');
        const runInfo = div.querySelector('div.run-info');

        engine.innerText = rpaData;
    }
}

const getRpaRunIds = () => {
    rpaData.keys()
}

const getRpaData = async () => {
    return await fetch('/api/getrpa')
}

const getFailedRpaData = async () => {

}

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