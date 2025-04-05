import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
    let users = await fetch("/svc/app-user").then(response => {
        console.log(response);

        return response.json();
    });
	console.log("users", users)

	return { app_users: users.app_users };
};