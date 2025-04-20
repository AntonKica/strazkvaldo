import { goto } from '$app/navigation';
import { base } from '$app/paths';
import { SVC_USER_ROOM } from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
    const rooms_response = await fetch(SVC_USER_ROOM.LIST());
    if (rooms_response.status == 401) {
        goto(`${base}/login`);
    }
    if (rooms_response.status != 200) {
        return {rooms: []};
    }

    const rooms = await rooms_response.json();

	return { rooms: rooms.rooms };
};