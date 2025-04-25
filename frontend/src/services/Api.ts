/* eslint-disable */
/* tslint:disable */
// @ts-nocheck
/*
 * ---------------------------------------------------------------
 * ## THIS FILE WAS GENERATED VIA SWAGGER-TYPESCRIPT-API        ##
 * ##                                                           ##
 * ## AUTHOR: acacode                                           ##
 * ## SOURCE: https://github.com/acacode/swagger-typescript-api ##
 * ---------------------------------------------------------------
 */

import {
  SalvoCoreHttpErrorsStatusErrorStatusError,
  ServerEmpty,
  ServerRoutersAuthLoginData,
  ServerRoutersAuthLoginOutData,
} from "./data-contracts";
import { ContentType, HttpClient, RequestParams } from "./http-client";

export class Api<
  SecurityDataType = unknown,
> extends HttpClient<SecurityDataType> {
  /**
   * No description
   *
   * @tags Auth
   * @name ServerRoutersAuthPostLogin
   * @request POST:/api/auth/login
   */
  serverRoutersAuthPostLogin = (
    data: ServerRoutersAuthLoginData,
    params: RequestParams = {},
  ) =>
    this.request<
      ServerRoutersAuthLoginOutData,
      SalvoCoreHttpErrorsStatusErrorStatusError
    >({
      path: `/api/auth/login`,
      method: "POST",
      body: data,
      type: ContentType.Json,
      format: "json",
      ...params,
    });
  /**
   * No description
   *
   * @tags Auth
   * @name ServerRoutersAuthGetLogout
   * @request GET:/api/auth/logout
   */
  serverRoutersAuthGetLogout = (params: RequestParams = {}) =>
    this.request<ServerEmpty, SalvoCoreHttpErrorsStatusErrorStatusError>({
      path: `/api/auth/logout`,
      method: "GET",
      format: "json",
      ...params,
    });
}
