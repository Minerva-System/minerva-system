import { faker } from '@faker-js/faker'; 


const coin = () => Math.random() > 0.5;

export const generateUser = () => {
    const firstName = faker.name.firstName();
    const lastName = faker.name.lastName();
    const login = faker.internet.userName(firstName, lastName);
    const password = faker.internet.password();
    const email = coin() ? faker.internet.email(firstName, lastName) : null;

    return {
        login,
        name: `${firstName} ${lastName}`,
        email,
        password
    };
};

export const updateUser = user => {
    const newFirstName = faker.name.firstName();
    const newLastName = faker.name.lastName();

    user.name = coin() ? user.name : `${newFirstName} ${newLastName}`;
    user.email = coin() ? user.email : (coin() ? faker.internet.email(newFirstName, newLastName) : null);
    user.password = coin() ? null : faker.internet.email(newFirstName, newLastName);

    return {
        login: user.login,
        name: user.name,
        email: user.email,
        password: user.password
    };
};
