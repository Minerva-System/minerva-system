import http from 'k6/http';
import { sleep, check } from 'k6';
import { describe, expect } from 'https://jslib.k6.io/k6chaijs/4.3.4.2/index.js';
import { htmlReport } from "https://raw.githubusercontent.com/benc-uk/k6-reporter/main/dist/bundle.js";
import { textSummary } from "https://jslib.k6.io/k6-summary/0.0.1/index.js";

import { generateUser, updateUser } from './generators.js';

export const options = {
    stages: [
        { duration: '2m', target:    1 }, // below normal load
        { duration: '5m', target:    1 },
        { duration: '2m', target:   40 }, // normal load
        { duration: '5m', target:   40 },
        { duration: '2m', target:   80 }, // around the breaking point
        { duration: '5m', target:   80 },
        { duration: '2m', target:  100 }, // beyond the breaking point
        { duration: '5m', target:  100 },
        { duration: '10m', target:   0 }, // scale down. Recovery stage.
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
    let token;
    let status;
    let user;
    
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
        expect(response).to.have.validJsonBody();

        const returned = response.json();
        expect(returned).to.be.an('object');

        expect(returned.tenant).to.exist;
        expect(returned.token).to.exist;
        expect(returned.tenant).to.equal(TENANT);

        token = `Bearer ${returned.token}`;
    });

    if (token === undefined) {
        console.warn(`Test interrupted: API returned status ${status}`);
    } else  {
        describe('Fetch user list', (t) => {
            const response = http.get(ROUTES.user, {
                headers: { 'Authorization': token }
            });

            expect(response.status).to.equal(200);
            expect(response).to.have.validJsonBody();

            const returned = response.json();
            expect(returned).to.be.an('array');
            expect(returned).to.have.lengthOf.above(0);
            
            const first = returned[0];
            expect(first.id).to.exist;
            expect(first.login).to.exist;
            expect(first.name).to.exist;
        });

        describe('Create user', (t) => {
            const newUser = generateUser();

            const payload = JSON.stringify(newUser);

            const response = http.post(ROUTES.user, payload, {
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': token
                }
            });

            status = response.status;
            expect(response.status).to.equal(200);
            expect(response).to.have.validJsonBody();

            const returned = response.json();
            expect(returned).to.be.an('object');

            if(returned.name !== newUser.name) {
                throw 'Created user name is not the same';
            }

            if(returned.email !== newUser.email) {
                throw 'Created user email is not the same';
            }

            if(returned.login !== newUser.login) {
                throw 'Created user login is not the same';
            }
            
            expect(returned.id).to.be.above(0);

            user = returned;
        });

        if(user === undefined) {
            console.warn(`Not testing user-related operations: API returned status ${status}`);
        } else {
            describe('Get user', (t) => {
                const response = http.get(`${ROUTES.user}/${user.id}`, {
                    headers: { 'Authorization': token }
                });

                expect(response.status).to.equal(200);
                expect(response).to.have.validJsonBody();

                const returned = response.json();
                expect(returned).to.be.an('object');

                if(returned.id !== user.id) {
                    throw 'Retrieved user ID is not the same';
                }
                
                if(returned.name !== user.name) {
                    throw 'Retrieved user name is not the same';
                }

                if(returned.email !== user.email) {
                    throw 'Retrieved user email is not the same';
                }

                if(returned.login !== user.login) {
                    throw 'Retrieved user login is not the same';
                }
            });

            describe('Update user', (t) => {
                const updatedUser = updateUser(user);

                const payload = JSON.stringify(updatedUser);

                const response = http.put(`${ROUTES.user}/${user.id}`, payload, {
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': token
                    }
                });

                status = response.status;
                expect(response.status).to.equal(200);
                expect(response).to.have.validJsonBody();

                const returned = response.json();
                expect(returned).to.be.an('object');

                if(returned.name !== updatedUser.name) {
                    throw 'Updated user name is not the same';
                }

                if(returned.email !== updatedUser.email) {
                    throw 'Updated user email is not the same';
                }

                if(returned.login !== updatedUser.login) {
                    throw 'Updated user login is not the same';
                }
                
                expect(returned.id).to.be.above(0);

                if(returned.id !== user.id) {
                    throw 'Updated user ID is not the same';
                }

                user = returned;
            });
            
            describe('Delete user', (t) => {
                const response = http.del(`${ROUTES.user}/${user.id}`, null, {
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': token
                    }
                });

                expect(response.status).to.equal(200);
                expect(response).to.have.validJsonBody();

                const returned = response.json();
                expect(returned).to.be.an('object');
                expect(returned.message).to.exist;
            });

            describe('Get user (deleted)', (t) => {
                const response = http.get(`${ROUTES.user}/${user.id}`, {
                    headers: { 'Authorization': token }
                });

                expect(response.status).to.equal(404);
                expect(response).to.have.validJsonBody();

                const returned = response.json();
                expect(returned).to.be.an('object');
                expect(returned.message).to.exist;

                user = undefined;
            });
        }

        describe('Perform logoff', (t) => {
            const response = http.post(ROUTES.logoff, null, {
                headers: { 'Authorization': token }
            });

            token = undefined;
            expect(response.status).to.equal(200);
            expect(response).to.have.validJsonBody();

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
