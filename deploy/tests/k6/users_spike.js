import http from 'k6/http';
import { sleep, check, fail } from 'k6';
import faker from 'k6/x/faker'; // Requires Faker k6 extension
import { htmlReport } from "https://raw.githubusercontent.com/benc-uk/k6-reporter/main/dist/bundle.js";
import { textSummary } from "https://jslib.k6.io/k6-summary/0.0.1/index.js";

export const options = {
    stages: [
        { duration: '2m', target: 10 }, // below normal load
        { duration: '5m', target: 10 },
        { duration: '2m', target: 20 }, // normal load
        { duration: '5m', target: 20 },
        { duration: '2m', target: 30 }, // around the breaking point
        { duration: '5m', target: 30 },
        { duration: '2m', target: 40 }, // beyond the breaking point
        { duration: '5m', target: 40 },
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

const setupCookies = (data) => {
    let jar = http.cookieJar();
    Object.keys(data.cookies).forEach(key => {
        jar.set(HOST, key, data.cookies[key][0]);
    });
};

const createFakeUser = () => {
    const name = faker.name();
    const email = faker.email();
    const password = "123123";
    const login = name.toLowerCase().replace(/\s/g, '');

    return { login, name, email, password };
};

// ==================================================== //

export function setup() {
    console.log('Logging in...');
    let res = performLogin();
    if(!check(res, {
        "login performed successfully": (r) => r.status === 200,
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
    })) {
        fail(`Unable to perform login: ${res.status}: ${res.body}`);
    }

    // Recover cookies
    const cookies = http.cookieJar().cookiesForURL(HOST);

    
    console.log('Retrieving users list...');
    res = http.get(route('/user'));
    if(!check(res, {
        "user list could be retrieved": (r) => r.status === 200,
    })) {
        fail(`Unable to get user list: ${res.status}: ${res.body}`);
    }
    const users = res.json()
          .filter(u => u.login !== 'admin');

    
    console.log('Removing all non-admin users...');
    const responses = users
          .map(u => http.del(route(`/user/${u.id}`)).status === 200);

    if(!check(responses, {
        "removed all users but administrator": (responses) => responses.every(r => r)
    })) {
        fail('Unable to remove all previous users');
    }

    console.log(`All ${users.length} previous users removed, starting tests in 5 seconds.`);
    sleep(5);

    return { cookies };
};

export default (data) => {
    setupCookies(data);

    let newUser, res;

    // Create user
    while(true) {
        newUser = createFakeUser();
        res = http.post(route('/user'), JSON.stringify(newUser));
        
        if(res.status === 412) {
            console.log('Fake user generated duplicated data, retrying');
        } else {
            break;
        }
    }

    if(!check(res, {
        "user creation succeeded": r => r.status === 200,
        "no cluster errors on creation": r => (Math.trunc(r.status / 100) < 5),
    })) {
        fail(`Unable to create user: ${res.status}: ${res.body}`);
    }
    
    if(!check(res, {
        "created user matches new user data": (r) => {
            const created = r.json();
            return (created !== undefined)
                && (created.login === newUser.login)
                && (created.name === newUser.name)
                && (created.email === newUser.email)
                && (created.password === undefined);
        },
        "created user ID is valid": (r) => r.json().id > 0,
    })) {
        fail(`Unable to create user: ${res.status}: ${res.body}`);
    }
    const createdUser = res.json();

    // Get user
    res = http.get(route(`/user/${createdUser.id}`));
    if(!check(res, {
        "created user retrieval succeeded": r => r.status === 200,
        "no cluster errors on retrieval": r => (Math.trunc(r.status / 100) < 5),
    })) {
        fail(`Unable to get user #${createdUser.id}: ${res.status}: ${res.body}`);
    }
    
    if(!check(res, {
        "retrieved user data matches created user": (r) => {
            const recovered = r.json();
            return (recovered !== undefined)
                && (recovered.login === createdUser.login)
                && (recovered.name === createdUser.name)
                && (recovered.email === createdUser.email)
                && (recovered.password === undefined);
        },
    })) {
        fail(`Unable to get user #${createdUser.id}: ${res.status}: ${res.body}`);
    }

    // Remove user
    res = http.del(route(`/user/${createdUser.id}`));
    if(!check(res, {
        "user removal succeeded": r => r.status === 200,
        "no cluster errors on removal": r => (Math.trunc(r.status / 100) < 5),
    })) {
        fail(`Unable to delete user #${createdUser.id}: ${res.status}: ${res.body}`);
    }

    // Get user (should fail)
    res = http.get(route(`/user/${createdUser.id}`));
    if(!check(res, {
        "removed user must not be retrieved": r => r.status === 404,
        "no cluster errors on retrieval": r => (Math.trunc(r.status / 100) < 5),
    })) {
        fail('Recovering deleted user was still possible!!!');
    }
    
    // Get users list
    res = http.get(route('/user'));
    if(!check(res, {
        "user listing succeeded": r => r.status === 200,
        "no cluster errors on listing": r => (Math.trunc(r.status / 100) < 5),
    })) {
        fail(`Unable to get user list: ${res.status}: ${res.body}`);
    }

    sleep(1);
};

export function teardown(data) {
    setupCookies(data);
    
    console.log('Performing logout...');
    const res = http.post(route('/logout'), null);
    if(!check(res, {
        "logout performed successfully": (r) => r.status === 200,
        "no cluster errors on logout": r => (Math.trunc(r.status / 100) < 5),
    })) {
        fail(`Unable to remove session: ${res.status}: ${res.body}`);
    }
};

export function handleSummary(data) {
  return {
    "users_spike.html": htmlReport(data),
    stdout: textSummary(data, { indent: " ", enableColors: true }),
  };
};
