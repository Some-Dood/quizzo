import { Discord } from 'deps';

// Command Implementations
import { leaderboard } from './leaderboard.ts';
import { ping } from './ping.ts';
import { start } from './start.ts';

export interface Command {
    readonly help: {
        readonly description: string;
        readonly usage: string;
    };
    execute(msg: Discord.Message, args: string[]): Promise<void>;
}

/** Command registry. */
const commands = new Map<string, Command>([
    [ 'leaderboard', leaderboard ],
    [ 'ping', ping ],
    [ 'start', start ],
]);

/** Queries for the given command name. */
export function getCommand(key: string) {
    if (key === 'help')
        throw new Error('not yet implemented');
    return commands.get(key);
}
