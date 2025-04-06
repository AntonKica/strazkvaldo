import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
    let user = await fetch(`/svc/app-user/${params.code}`).then(response => {
        return response.json();
    });

    return {
        user: user.data.user,
        roles: [0, 1]
    };
};