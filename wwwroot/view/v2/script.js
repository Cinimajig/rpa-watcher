const intervalSeconds = 5;

let rpaDataLink = '/api/gettemplate';
let historyRpaDataLink = '/api/gethistorytemplate?amount=50';

const subflow = `<img src="../Down_Right.svg" alt="Child flow" class="image" />`;
const defaultLogo = `<img src="../parent.svg" alt="Unknown engine or child flow" class="image" />`;
let paLogo = `<img src="../PALogo.png" alt="Power Automate" class="image" />`;
let prLogo = `<img src="../PRLogo.png" alt="ProcessRobot" class="image" />`;
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
                document.body.classList.add('dark');
            }
            break;
    }
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

        sortedRuns() {
            return [...this.running].sort((a, b) => a[1].started > b[1].started ? 1 : -1);
        },

        async retrieveData() {
            const data = await (await fetch(rpaDataLink)).json();
            const history = await (await fetch(historyRpaDataLink)).json();

            this.running = data;
            // this.running.clear();
            // for (let item of data) {
            //     // this.running.set(item.instance, item);
            // }

            this.history = history;
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
                    return dt.toLocaleString(timeZone);
                } else {
                    return dt.toLocaleString();
                }
            } catch {
                return '';
            }
        },

        // This code will be executed before Alpine
        // initializes the rest of the component.
        init() {
            this.retrieveData().catch(err => {
                this.lastError = err;
            });

            setInterval(() => {
                this.retrieveData().then(() => this.lastError = '').catch(err => {
                    this.lastError = err;
                });
            }, intervalSeconds * 1000);
        }
    }));


});
