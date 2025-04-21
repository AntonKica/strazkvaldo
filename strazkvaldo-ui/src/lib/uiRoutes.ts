import { base } from '$app/paths';

class EntityRouteUI {
    roleName: string;
    entityName: string;

    constructor(roleName: string, entityName: string) {
        this.roleName = roleName;
        this.entityName = entityName;
    }

    basePath(): string {
        return `${base}/${this.roleName}/${this.entityName}`
    }
    LIST(): string {
        return this.basePath();
    }
    VIEW(code: string): string {
        return `${this.basePath()}/${code}/view`;
    }
    EDIT(code: string): string {
        return `${this.basePath()}/${code}/edit`;
    }
    CREATE(): string {
        return `${this.basePath()}/create`;
    }
};

export const UI_ADMIN_APP_USER = new EntityRouteUI('admin', 'app-user');
export const UI_USER_ONE_TIME_ACTIVITY = new EntityRouteUI('user', 'one-time-activity');
export const UI_USER_REPEATED_ACTIVITY = new EntityRouteUI('user', 'repeated-activity');
export const UI_USER_ROOM = new EntityRouteUI('user', 'room');