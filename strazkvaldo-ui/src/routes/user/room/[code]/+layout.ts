import { SVC_USER_ROOM, SVC_ENUM_ROOM_TYPE } from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
    let response = await fetch(SVC_USER_ROOM.GET(params.code)).then(response => {
        return response.json();
    });
    let room_type_response = await fetch(SVC_ENUM_ROOM_TYPE.LIST()).then(response => {
        return response.json();
    });

    return {
        room: response.data.room,
        room_types: room_type_response.enum_values,
    };
};