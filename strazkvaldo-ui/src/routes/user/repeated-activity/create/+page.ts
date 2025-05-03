import { SVC_ENUM_ACTIVITY_TYPE, SVC_ENUM_CRITICALITY_TYPE, SVC_ENUM_PERIODICITY, SVC_USER_ROOM } from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
    let activity_type_result = await fetch(SVC_ENUM_ACTIVITY_TYPE.LIST()).then(response => {
        return response.json();
    });
    let criticality_type_result = await fetch(SVC_ENUM_CRITICALITY_TYPE.LIST()).then(response => {
        return response.json();
    });
    let periodicity_result = await fetch(SVC_ENUM_PERIODICITY.LIST()).then(response => {
        return response.json();
    });
    const rooms_response = await fetch(SVC_USER_ROOM.LIST()).then(response => response.json());

    return {
        activity_types: activity_type_result.enum_values,
        criticality_types: criticality_type_result.enum_values,
        periodicity: periodicity_result.enum_values,
        rooms: rooms_response.rooms,
    };
};