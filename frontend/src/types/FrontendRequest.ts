// Code generated by jtd-codegen for TypeScript v0.2.1

export type FrontendRequest = FrontendRequestCommand | FrontendRequestPage | FrontendRequestToken;

export interface FrontendRequestCommand {
  requestType: "COMMAND";
  cmd: string;
  args?: string[];
}

export interface FrontendRequestPage {
  requestType: "PAGE";
  page: string;
}

export interface FrontendRequestToken {
  requestType: "TOKEN";
  token: string;
}
