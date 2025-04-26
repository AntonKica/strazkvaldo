import { base } from '$app/paths';

class EntityRoute {
    roleName: string;
    entityName: string;

    constructor(roleName: string, entityName: string) {
        this.roleName = roleName;
        this.entityName = entityName;
    }

    basePath(): string {
        return `/svc/${this.roleName}/${this.entityName}`
    }
    LIST(): string {
        return this.basePath();
    }
    POST(): string {
        return this.basePath();
    }

    GET(code: string): string {
        return `${this.basePath()}/${code}`;
    }
    PATCH(code: string): string {
        return `${this.basePath()}/${code}`;
    }
    CUSTOM_CODE(custom: string, code: string) {
        return `${this.basePath()}/${code}/${custom}`;
    }
};

export const SVC_ADMIN_APP_USER = new EntityRoute('admin', 'app-user');
export const SVC_USER_ONE_TIME_ACTIVITY = new EntityRoute('user', 'one-time-activity');
export const SVC_USER_REPEATED_ACTIVITY = new EntityRoute('user', 'repeated-activity');
export const SVC_USER_ROOM = new EntityRoute('user', 'room');
export const SVC_USER_UPCOMING_ACTIVITY = new EntityRoute('user', 'upcoming-activity');
export const SVC_USER_RECENTLY_FINISHED_ACTIVITY = new EntityRoute('user', 'recently-finished-activity');
export const SVC_USER_REVIEWED_FINISHED_ACTIVITY = new EntityRoute('user', 'reviewed-finished-activity');

export const SVC_ENUM_APP_USER_ROLE = new EntityRoute('enum', 'app-user-role');
export const SVC_ENUM_CRITICALITY_TYPE = new EntityRoute('enum', 'criticality-type');
export const SVC_ENUM_ROOM_TYPE = new EntityRoute('enum', 'room-type');
export const SVC_ENUM_ACTIVITY_TYPE = new EntityRoute('enum', 'activity-type');
export const SVC_ENUM_PERIODICITY = new EntityRoute('enum', 'periodicity');