import http from 'k6/http';
import { sleep, check } from 'k6';
import { htmlReport } from "https://raw.githubusercontent.com/benc-uk/k6-reporter/main/dist/bundle.js";
import { textSummary } from "https://jslib.k6.io/k6-summary/0.0.1/index.js";

export const options = {
    stages: [
        { duration: '2m', target: 100 }, // below normal load
        { duration: '5m', target: 100 },
        { duration: '2m', target: 200 }, // normal load
        { duration: '5m', target: 200 },
        { duration: '2m', target: 300 }, // around the breaking point
        { duration: '5m', target: 300 },
        { duration: '2m', target: 400 }, // beyond the breaking point
        { duration: '5m', target: 400 },
        { duration: '10m', target: 0 }, // scale down. Recovery stage.
    ],
};

// Requires environment variable MINERVA_HOST
const HOST = `${__ENV.MINERVA_HOST}`;
const BASE_URL = `${HOST}/api`;
const TENANT = 'teste';

const route = endpoint => `${BASE_URL}${endpoint}`;

const performLogin = () => {
    let data = {
        login: 'admin',
        password: 'admin',
    };
    
    return http.post(route(`/${TENANT}/login`), JSON.stringify(data), {
        headers: { 'Content-Type': 'application/json' },
    });
};

// ==================================================== //

const setup = () => {};

export default () => {
    // Login
    const login_response = performLogin();
    check(login_response, {
        "login performed successfully": (res) => res.status == 200,
        "login token was returned": (r) => {
            const token = r.json().token;
            return (token !== undefined)
                && (token.length > 0);
        },
        "login tenant matches test tenant": (r) => {
            const tenant = r.json().tenant;
            return (tenant !== undefined)
                && (tenant === TENANT);
        },
    });

    // Logout
    const logout_response = http.post(route('/logout'), null, null);
    check(logout_response, {
        "logout performed successfully": (res) => res.status == 200,
    });
    
    sleep(1);
};

const teardown = () => {};

export function handleSummary(data) {
  return {
    "session_spike.html": htmlReport(data),
    stdout: textSummary(data, { indent: " ", enableColors: true }),
  };
};
