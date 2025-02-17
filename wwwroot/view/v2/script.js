const intervalSeconds = 5;

let rpaDataLink = '/api/getrpa';
let historyRpaDataLink = '/api/gethistory?amount=50';
// let rpaDataLink = '/api/gettemplate';
// let historyRpaDataLink = '/api/gethistorytemplate?amount=50';

const subflow = `<img src="../Down_Right.svg" alt="Child flow" class="image" />`;
const defaultLogo = `<img src="../parent.svg" alt="Unknown engine or child flow" class="image" />`;
let paLogo = `<img src="../PALogo.png" alt="Power Automate" class="image" />`;
let prLogo = `<img src="../PRLogo.png" alt="ProcessRobot" class="image" />`;
let darkMode = window.matchMedia("(prefers-color-scheme: dark)");
let timeZone = null;
let noBlue = false;
let noRed = false;

if (darkMode) {
    document.body.setAttribute("data-theme", 'dark');
}

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
                document.body.setAttribute("data-theme", 'dark');
            }
            break;
        case 'dark':
            if (value ? parseBool(value) : true) {
                document.body.setAttribute("data-theme", 'light');
            }
            break;
    }
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
            if (this.lastError) {
                return true;
            }
            return false;
        },

        sortRuns(list) {
            // list.sort((a, b) => a.started > b.started ? 1 : -1);
            list.sort((a, b) => {
                if (!a.parentInstance) {
                    return -5;
                }
                if (a.parentInstance === b.parentInstance) {
                    return -4;
                }

                if (a.started < b.started) {
                    return -3;
                }

                return 1;
            });
        },

        async retrieveData() {
            const data = await fetch(rpaDataLink);
            const history = await fetch(historyRpaDataLink);

            this.running = await data.json();
            this.sortRuns(this.running);
            // this.running.clear();
            // for (let item of data) {
            //     // this.running.set(item.instance, item);
            // }

            this.history = await history.json();
            this.sortRuns(this.history);
            // this.history.clear();
            // for (let item of history) {
            //     this.history.set(item.instance, item);
            // }
        },

        engineLogo(str) {
            switch (str) {
                case 'Power Automate':
                    return paLogo;
                case 'ProcessRobot':
                    return prLogo;
                default:
                    return defaultLogo;
            }
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

        // This code will be executed before Alpine
        // initializes the rest of the component.
        init() {
            this.retrieveData()
            .catch(err => {
                this.lastError = err;
            });

            setInterval(() => {
                this.retrieveData()
                .then(() => this.lastError = '')
                .catch(err => {
                    this.lastError = err;
                });
            }, intervalSeconds * 1000);
        }
    }));


});
