import type { PageLoad } from './$types';
import { SVC_ADMIN_APP_USER } from '$lib/serviceRoutes';

export const load: PageLoad = async ({ fetch }) => {
    let users = await fetch(SVC_ADMIN_APP_USER.LIST()).then(response => {
        console.log(response);

        return response.json();
    });
	return users;
};