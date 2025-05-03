import { SVC_ENUM_ROOM_TYPE } from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
    let room_type_response = await fetch(SVC_ENUM_ROOM_TYPE.LIST()).then(response => {
        return response.json();
    });

    return {
        room_types: room_type_response.enum_values,
    };
};