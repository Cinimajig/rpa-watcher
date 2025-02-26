const intervalSeconds = 5;

let rpaDataLink = '/api/getrpa';
let historyRpaDataLink = '/api/gethistory?amount=50';
let testMode = false;

// const subflow = `<img src="../Down_Right.svg" alt="Child flow" class="image" />`;
const subflow = `<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m16.49 12 3.75 3.75m0 0-3.75 3.75m3.75-3.75H3.74V4.499" /></svg>`;
const defaultLogo = `<img src="../parent.svg" alt="Unknown engine or child flow" class="image" />`;
let paLogo = `<img src="../PALogo.png" alt="Power Automate" class="image" />`;
let prLogo = `<img src="../PRLogo.png" alt="ProcessRobot" class="image" />`;
let darkMode = window.matchMedia("(prefers-color-scheme: dark)").matches;
let timeZone = null;
let noBlue = false;
let noRed = false;

const parseBool = (str) => str.trim().toLowerCase() == 'true';

const parseTrigger = (str) => {
    const prefix = 'Started from Console by';
    if (str.startsWith(prefix)) {
        return str.slice(prefix.length).trim();
    }
    return str;
}

for (let param of window.location.search.substring(1).split('&')) {
    const [name, value] = param.split('=');

    switch (name) {
        case 'tz':
            timeZone = value.trim();
            break;
        case 'no-color':
            if (value ? parseBool(value) : true) {
                prLogo = `<img src="../PRLogo_GS.png" alt="ProcessRobot" class="image" />`;
                paLogo = `<img src="../PALogo_GS.png" alt="Power Automate" class="image" />`;
                noRed = true;
                noBlue = true;
            }
            break;
        case 'no-red':
            if (value ? parseBool(value) : true) {
                prLogo = `<img src="../PRLogo_GS.png" alt="ProcessRobot" class="image" />`;
                noRed = true;
            }
            break;
        case 'no-blue':
            if (value ? parseBool(value) : true) {
                paLogo = `<img src="../PALogo_GS.png" alt="Power Automate" class="image" />`;
                noBlue = true;
            }
            break;
        case 'dark':
            if (value ? parseBool(value) : true) {
                // document.body.setAttribute("data-theme", 'dark');
                darkMode = true;
            }
            break;
        case 'light':
            if (value ? parseBool(value) : true) {
                // document.body.setAttribute("data-theme", 'light');
                darkMode = false;
            }
            break;
        case 'test':
            if (value ? parseBool(value) : true) {
                testMode = true;
                rpaDataLink = '/api/gettemplate';
                historyRpaDataLink = '/api/gethistorytemplate?amount=50';
            }
    }
}

if (darkMode) {
    document.body.setAttribute("data-theme", 'dark');
} else {
    document.body.setAttribute("data-theme", 'light');
}


const getdata = async (uri) => {
    const res = await fetch(uri);
    return await res.text();
}

document.addEventListener('alpine:init', () => {
    Alpine.data('rpaData', () => ({
        running: [],
        history: [],
        lastError: '',

        hasError() {
            return this.lastError ? true : false;
        },

        sortRuns(list) {
            list.sort((a, b) => a.started > b.started ? 1 : -1);
            // Minus == a before b
            // Positive == a after b
            // Zero == the same
            // list.sort((a, b) => {
                // if (!a.parentInstance) {
                //     return -5;
                // }
                // if (a.parentInstance === b.parentInstance) {
                //     return -4;
                // }

            //     if (a.started < b.started) {
            //         return -3;
            //     }

            //     return 1;
            // });
        },

        // Resursive.
        runsWithParent(parentInstance) {
            let runs = this.running.filter(run => run.parentInstance === parentInstance.instance);

            for (let run of runs) {
                runs = runs.concat(this.runsWithParent(run));
            }

            console.log(runs);
            
            return runs;
        },

        runsWithoutParents() {
            return this.running.filter(run => !run.parentInstance);
        },

        async retrieveData() {
            const [data, history] = await Promise.all([fetch(rpaDataLink), fetch(historyRpaDataLink)]);

            this.running = await data.json();
            this.sortRuns(this.running);
            // this.running.clear();
            // for (let item of data) {
            //     // this.running.set(item.instance, item);
            // }

            this.history = await history.json();
            // this.history.clear();
            // for (let item of history) {
            //     this.history.set(item.instance, item);
            // }
        },

        engineLogo(run, useDefault) {
            if (!!run.parentInstance && useDefault) {
                return subflow;
            }

            switch (run.engine) {
                case 'Power Automate':
                    return paLogo;
                case 'ProcessRobot':
                    return prLogo;
                default:
                    return defaultLogo;
            }
        },

        flowName(run) {
            return run.name ? run.name : run.instance;
        },

        localDate(str) {
            try {
                const dt = new Date(str);
                if (timeZone) {
                    return dt.toLocaleString(timeZone).replace(',', '');
                } else {
                    return dt.toLocaleString().replace(',', '');
                }
            } catch {
                return '';
            }
        },

        exist(item) {
            return !!item;
        },

        resetError() {
            console.log(testMode);
            testMode ? this.lastError = 'Error: This is site is in test mode!' : this.lastError = '';
        },

        // This code will be executed before Alpine
        // initializes the rest of the component.
        init() {
            if (testMode) { this.resetError(); }

            this.retrieveData()
            .catch(err => {
                this.lastError = err;
            });

            setInterval(() => {
                this.retrieveData()
                .then(() => this.resetError())
                .catch(err => {
                    this.lastError = err;
                });
            }, intervalSeconds * 1000);
        }
    }));


});
