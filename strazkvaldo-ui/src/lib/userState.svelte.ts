import { AppUserRole } from "./common";

type UserState = {
    signedIn: boolean;
    username: String | null;
    code: string | null;
    role: AppUserRole;
};

let userState = $state<UserState>({
    signedIn: false,
    username: null, 
    code: null,
    role: AppUserRole.Unsigned
});

function persistUserState() {
    if ( typeof window !== undefined) {
        localStorage.setItem('userState', JSON.stringify(userState))
    }
}

export const auth = {
    get userState(): UserState {
        return userState;
    },
    login(username: String,
        code: string,
        role: AppUserRole) {
        userState.signedIn = true;
        userState.username = username;
        userState.code = code;
        userState.role = role;
        persistUserState();
    },
    logout() {
        userState.signedIn = false;
        userState.username = null
        userState.code = null
        userState.role = AppUserRole.Unsigned;
        if ( typeof window !== undefined) {
            localStorage.removeItem('userState');
        }
    }
};

if (typeof window !== undefined) {
    const stored = localStorage.getItem('userState');
    if (stored) {
        const storedUserState: UserState = JSON.parse(stored);
        userState.signedIn = storedUserState.signedIn;
        userState.username = storedUserState.username;
        userState.code = storedUserState.code;
        userState.role = storedUserState.role;
    }
}