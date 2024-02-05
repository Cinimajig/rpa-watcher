const timeoutSeconds = 7;
const rpaView = document.querySelector('div#rpa-view');

let rpaData = [];
let failedRpaData = [];

setTimeout(buildRpaConvas, timeoutSeconds * 1000);

const buildRpaConvas = async () => {
    rpaData = await getRpaData();
    failedRpaData = await getFailedRpaData();

    const template = document.querySelector('div.rpa-info');
}

const getRpaData = async () => {

}

const getFailedRpaData = async () => {

}
