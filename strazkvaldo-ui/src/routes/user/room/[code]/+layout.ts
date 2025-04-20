import { SVC_USER_ROOM } from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
    let response = await fetch(SVC_USER_ROOM.GET(params.code)).then(response => {
        return response.json();
    });

    return {
        room: response.data.room,
        room_types: [
            {code: 0, name: "kúpeľňa"},
            {code: 1, name: "obývačka"},
        ],
    };
};