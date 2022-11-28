import http from 'k6/http';
import { sleep, check } from 'k6';
import { describe, expect } from 'https://jslib.k6.io/k6chaijs/4.3.4.2/index.js';
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

const value_or = (value, default_val) => value === undefined ? default_val : value;

const HOST = value_or(__ENV.MINERVA_HOST, 'http://localhost:9000/api');
const TENANT = value_or(__ENV.MINERVA_TENANT, 'teste');

const BASE_URL = `${HOST}/${TENANT}`;

// routes
const ROUTES = {
    login: `${BASE_URL}/login`,
    logoff: `${BASE_URL}/logout`,
    user: `${BASE_URL}/user`,
};

export const setup = () => {
    console.info(`Performing operations in ${HOST} with tenant ${TENANT}...`);
};

export default () => {
    let token = '';
    let status;
    
    describe('Perform login', (t) => {
        const payload = JSON.stringify({
            login: 'admin',
            password: 'admin'
        });

        const response = http.post(ROUTES.login, payload, {
            headers: { 'Content-Type': 'application/json' }
        });

        status = response.status;

        expect(response.status).to.equal(200);

        const returned = response.json();

        expect(returned).to.be.an('object');
        expect(returned.tenant).to.exist;
        expect(returned.token).to.exist;
        expect(returned.tenant).to.equal(TENANT);

        token = `Bearer ${returned.token}`;
    });

    if (token === '') {
        console.warn(`Test interrupted: API returned status ${status}`);
    } else  {
        describe('Fetch user list', (t) => {
            const response = http.get(ROUTES.user, {
                headers: { 'Authorization': token }
            });

            expect(response.status).to.equal(200);

            const returned = response.json();
            expect(returned).to.be.an('array');
            expect(returned).to.have.lengthOf.above(0);
            
            const first = returned[0];
            expect(first.id).to.exist;
            expect(first.login).to.exist;
            expect(first.name).to.exist;
        });

        describe('Perform logoff', (t) => {
            const response = http.post(ROUTES.logoff, null, {
                headers: { 'Authorization': token }
            });

            token = '';
            expect(response.status).to.equal(200);

            const returned = response.json();
            expect(returned.message).to.exist;
        });
    }
};

export const handleSummary = (data) => {
  return {
    "stress_test.html": htmlReport(data),
    stdout: textSummary(data, { indent: " ", enableColors: true }),
  };
};
